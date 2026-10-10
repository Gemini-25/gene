//! Register VM and the read-only Think stage. SPEC §8, §9.

use crate::agent::Agent;
use crate::intent::{attempt_cost, dom, rotate, unpack, CopyCapture, Fault, Intent, Outcome, OFFSETS};
use crate::isa::{decode, op};
use crate::rng::Stream;
use crate::media::Media;
use crate::world::World;
use std::collections::BTreeMap;

pub const FUEL: u32 = 32;
pub const INSTR_COST: u32 = 4;
pub const QUERY_COST: u32 = 8;
pub const MAX_QUERIES: u8 = 4;
pub const MAX_FRAMES: usize = 8;
pub const SENSE_RADIUS: i32 = 2;

/// Publicly visible face of an actor (readable via ActorSurface).
#[derive(Clone, Copy, Debug)]
pub struct ActorFace {
    pub uid: u64,
    pub orient: u8,
    pub shell: bool,
    pub port: u16,
}

pub struct Sense<'a> {
    pub world: &'a World,
    pub faces: &'a [ActorFace],
    pub tick: u64,
    /// M1a capabilities (signals, public tape, drafts).
    pub m1: bool,
    /// M1c parcels.
    pub parcels: bool,
    pub signal: &'a BTreeMap<u64, i16>,
    pub dropped: &'a BTreeMap<u32, Vec<Media>>,
}

impl Sense<'_> {
    /// Public record on tile t by selector (1-based, UID order). Selector 0 (parcel media) is absent in M1a.
    pub fn public(&self, t: u32, selector: u16) -> Option<&Media> {
        if selector == 0 {
            return None;
        }
        self.dropped.get(&t)?.get(selector as usize - 1)
    }

    fn face_at(&self, t: u32) -> Option<&ActorFace> {
        match self.world.occ[t as usize] {
            0 => None,
            i => Some(&self.faces[(i - 1) as usize]),
        }
    }
}

fn sense_tile(a: &Agent, w: &World, dx: i16, dy: i16, radius: i32) -> Result<u32, Fault> {
    let (dx, dy) = (dx as i32, dy as i32);
    if dx.abs() + dy.abs() > radius {
        return Err(Fault::BadAddress);
    }
    let me = w.idx(a.x as u32, a.y as u32);
    let (ax, ay) = rotate(a.orient, dx, dy);
    let t = w.offset(me, ax, ay).ok_or(Fault::Boundary)?;
    // distance 2: visible if one of the axis-order shortest paths has an open middle tile
    if dx.abs() + dy.abs() == 2 && !w.parcels.is_empty() {
        let mids: &[(i32, i32)] = if dx == 0 || dy == 0 { &[(dx / 2, dy / 2)] } else { &[(dx, 0), (0, dy)] };
        let open = mids.iter().any(|&(mx, my)| {
            let (rx, ry) = rotate(a.orient, mx, my);
            w.offset(me, rx, ry).is_some_and(|m| !w.blocked(m))
        });
        if !open {
            return Err(Fault::Occluded);
        }
    }
    Ok(t)
}

fn read(a: &mut Agent, s: &Sense) -> Result<u16, Fault> {
    let r = a.vm.regs;
    let (domain, field, index) = (r[0], r[3], r[4]);
    let w = s.world;
    let bad = Err(Fault::BadAddress);
    match domain {
        dom::SELF => Ok(match field {
            0 => a.energy.min(65535) as u16,
            1 => {
                if index >= 10 {
                    return bad;
                }
                a.inv[index as usize]
            }
            2 => a.age as u16,
            3 => (a.age >> 16) as u16,
            4 => a.orient as u16,
            5 => {
                let f = a.vm.fault;
                a.vm.fault = Fault::Ok as u8;
                f as u16
            }
            6 => a.embryo_state_code(),
            7 => a.used_len as u16,
            8 => 0,
            9 => a.port_mask,
            10 => a.last_actual,
            11 => a.body_g,
            12 => a.shell_s,
            _ => return bad,
        }),
        dom::LOOSE => {
            let t = sense_tile(a, w, r[1] as i16, r[2] as i16, SENSE_RADIUS)?;
            if field != 0 || index >= 10 {
                return bad;
            }
            Ok(w.loose_visible(t, index as usize))
        }
        dom::PARCEL => {
            let t = sense_tile(a, w, r[1] as i16, r[2] as i16, SENSE_RADIUS)?;
            let p = if s.parcels { w.parcels.get(&t) } else { None };
            match (field, p) {
                (6, p) => Ok(p.is_some() as u16),
                (_, None) => bad,
                (0, Some(p)) if index < 10 => Ok(p.matrix[index as usize]),
                (1, Some(p)) => Ok(p.c as u16),
                (2, Some(_)) => Ok(0),
                (3, Some(_)) if index < 4 => Ok(0),
                (4, Some(_)) => Ok(0),
                (5, Some(p)) => Ok(p.capacity() as u16),
                _ => bad,
            }
        }
        dom::PORE if s.parcels => {
            let t = sense_tile(a, w, r[1] as i16, r[2] as i16, SENSE_RADIUS)?;
            let p = w.parcels.get(&t).ok_or(Fault::BadAddress)?;
            match field {
                0 if index < 10 => Ok(p.pore[index as usize]),
                1 => Ok(p.capacity() as u16),
                _ => bad,
            }
        }
        dom::ACTOR => {
            let t = sense_tile(a, w, r[1] as i16, r[2] as i16, SENSE_RADIUS)?;
            let face = s.face_at(t);
            match (field, face) {
                (0, f) => Ok(f.is_some() as u16),
                (_, None) => bad,
                (1, Some(f)) => Ok(f.orient as u16),
                (2, Some(f)) => Ok(f.shell as u16),
                (3, Some(f)) => Ok(f.port),
                _ => bad,
            }
        }
        dom::OWN_CODE => {
            let Some(m) = a.bank_media(r[6]) else { return bad };
            match field {
                0 => m.read(index as u32).ok_or(Fault::BadAddress),
                1 => Ok(m.logical_len.min(65535) as u16),
                2 => Ok(m.capacity().min(65535) as u16),
                _ => bad,
            }
        }
        dom::OWN_DRAFT => match (field, a.draft.as_ref()) {
            (0, Some(d)) => d.read(index as u32).ok_or(Fault::BadAddress),
            (1, d) => Ok(d.map_or(0, |d| d.logical_len.min(65535) as u16)),
            (2, d) => Ok(d.map_or(0, |d| d.capacity().min(65535) as u16)),
            (3, d) => Ok(d.and_then(|d| d.recycle).map_or(0, |(_, n)| 128 - n)),
            _ => bad,
        },
        dom::SIGNAL if s.m1 => {
            let t = sense_tile(a, w, r[1] as i16, r[2] as i16, SENSE_RADIUS)?;
            if field != 0 || index >= 4 {
                return bad;
            }
            a.s.signal_reads = a.s.signal_reads.saturating_add(1);
            Ok(s.signal.get(&crate::sim::signal_key(t, index)).copied().unwrap_or(0) as u16)
        }
        dom::PUBLIC_TAPE if s.m1 => {
            let t = sense_tile(a, w, r[1] as i16, r[2] as i16, SENSE_RADIUS)?;
            let m = s.public(t, r[6]).ok_or(Fault::BadAddress)?;
            a.s.public_reads = a.s.public_reads.saturating_add(1);
            match field {
                0 => m.read(index as u32).ok_or(Fault::BadAddress),
                1 => Ok(m.logical_len.min(65535) as u16),
                2 => Ok(m.capacity().min(65535) as u16),
                3 => m.present.get(index as usize).map(|p| *p as u16).ok_or(Fault::BadAddress),
                4 => Ok(m.recycle.map_or(0, |(_, n)| 128 - n)),
                _ => bad,
            }
        }
        dom::EMBRYO => {
            let e = a.embryo.as_ref();
            match field {
                0 => e.and_then(|e| e.raw.read(index as u32)).ok_or(Fault::BadAddress),
                1 => Ok(e.map_or(0, |e| e.raw.logical_len.min(65535) as u16)),
                2 => Ok(e.map_or(0, |e| e.raw.capacity().min(65535) as u16)),
                3 => Ok(a.embryo_state_code()),
                4 => Ok(e.map_or(0, |e| e.energy.min(65535) as u16)),
                5 => {
                    if index >= 10 {
                        return bad;
                    }
                    Ok(e.map_or(0, |e| e.ret[index as usize]))
                }
                _ => bad,
            }
        }
        dom::SURFACE => {
            let t = sense_tile(a, w, r[1] as i16, r[2] as i16, SENSE_RADIUS)?;
            Ok(match field {
                0 => w.elev[t as usize] as u16,
                1 => w.mu[t as usize],
                2 => w.surface_c[t as usize] as u16,
                3 => w.theta(t).min(65535) as u16,
                4 => w.light(t, s.tick).min(65535) as u16,
                _ => return bad,
            })
        }
        // Signal, PublicTape, LocalTrace, PoreStock: capability off in M0.
        _ => bad,
    }
}

fn capture_copy(a: &Agent, s: &Sense) -> Option<CopyCapture> {
    let r = a.vm.regs;
    let (sd, so) = unpack(r[0])?;
    let mut src_tile = None;
    let media = match (sd, so) {
        (dom::OWN_CODE, 0) => a.bank_media(r[7])?,
        (dom::EMBRYO, 0) => &a.embryo.as_ref()?.raw,
        (dom::OWN_DRAFT, 0) if s.m1 => a.draft.as_ref()?,
        (dom::PUBLIC_TAPE, off) if s.m1 => {
            let (dx, dy) = OFFSETS[off as usize];
            let (ax, ay) = rotate(a.orient, dx, dy);
            let t = s.world.offset(s.world.idx(a.x as u32, a.y as u32), ax, ay)?;
            src_tile = Some(t);
            s.public(t, r[7])?
        }
        _ => return None,
    };
    let count = r[6] as u32;
    let start = r[4] as u32;
    let mut bytes = Vec::new();
    for i in start..start.saturating_add(count.min(4)) {
        if i >= media.logical_len {
            break;
        }
        match media.read(i) {
            Some(v) => bytes.push(v),
            None => break,
        }
    }
    Some(CopyCapture { src_tile, src_uid: media.uid, src_rev: media.revision, bytes })
}

/// Parcel on the first parcel endpoint of a TRANSFER, or at the WORK position.
fn capture_parcel(a: &Agent, s: &Sense, opc: u8) -> Option<u64> {
    let r = a.vm.regs;
    let me = s.world.idx(a.x as u32, a.y as u32);
    let t = match opc {
        op::TRANSFER => {
            let (sd, so) = unpack(r[0])?;
            let (dd, doff) = unpack(r[1])?;
            let off = if matches!(sd, dom::PARCEL | dom::PORE) {
                so
            } else if matches!(dd, dom::PARCEL | dom::PORE) {
                doff
            } else {
                return None;
            };
            let (dx, dy) = OFFSETS[off as usize];
            let (ax, ay) = rotate(a.orient, dx, dy);
            s.world.offset(me, ax, ay)?
        }
        op::WORK if r[0] == dom::PARCEL => sense_tile(a, s.world, r[1] as i16, r[2] as i16, 1).ok()?,
        _ => return None,
    };
    s.world.parcels.get(&t).map(|p| p.uid)
}

fn capture_target(a: &Agent, s: &Sense) -> Option<u64> {
    let r = a.vm.regs;
    let (dd, off) = unpack(r[1])?;
    if dd != dom::ACTOR || off == 0 {
        return None;
    }
    let (dx, dy) = OFFSETS[off as usize];
    let (ax, ay) = rotate(a.orient, dx, dy);
    let t = s.world.offset(s.world.idx(a.x as u32, a.y as u32), ax, ay)?;
    s.face_at(t).map(|f| f.uid)
}

#[inline]
fn pay(a: &mut Agent, q: u32) {
    a.energy -= q;
    a.s.heat_pending += q as u64;
}

/// Run the agent's VM for one tick. Writes only the agent itself.
pub fn think(a: &mut Agent, s: &Sense, ctrl_key: [u32; 2]) {
    if let Some((o, act)) = a.receipt.take() {
        a.vm.regs[14] = o as u16;
        a.vm.regs[15] = act;
        a.last_actual = act;
    }
    let entry = a.entry();
    // code of the current slot, refreshed whenever the slot changes
    let (mut prog, mut mi) = a.slot_code(a.vm.module);
    let mut cur_slot = a.vm.module;
    let mut fuel = FUEL;
    while fuel > 0 {
        if a.vm.module != cur_slot {
            (prog, mi) = a.slot_code(a.vm.module);
            cur_slot = a.vm.module;
        }
        let m = prog.modules[mi];
        if a.vm.pc > m.len {
            a.vm.module = entry;
            a.vm.pc = 0;
            break;
        }
        if a.energy < INSTR_COST {
            a.vm.fault = Fault::EnergyStop as u8;
            break;
        }
        pay(a, INSTR_COST);
        fuel -= 1;
        if a.vm.pc == m.len {
            // implicit RET
            if let Some((mi, pc)) = a.vm.stack.pop() {
                a.vm.module = mi;
                a.vm.pc = pc;
                continue;
            }
            a.vm.module = entry;
            a.vm.pc = 0;
            break;
        }
        let word = prog.word(mi, a.vm.pc);
        let (opc, ra, rb, imm) = decode(word);
        let mut next = a.vm.pc + 1;
        match opc {
            op::NOP => {}
            op::CONST => a.vm.regs[ra] = imm,
            op::MOV => a.vm.regs[ra] = a.vm.regs[rb],
            op::ADD => a.vm.regs[ra] = a.vm.regs[ra].wrapping_add(a.vm.regs[rb]),
            op::SUB => a.vm.regs[ra] = a.vm.regs[ra].wrapping_sub(a.vm.regs[rb]),
            op::MUL => a.vm.regs[ra] = a.vm.regs[ra].wrapping_mul(a.vm.regs[rb]),
            op::DIV | op::REM => {
                if a.vm.regs[rb] == 0 {
                    a.vm.regs[ra] = 0;
                    a.vm.fault = Fault::DivZero as u8;
                } else if opc == op::DIV {
                    a.vm.regs[ra] /= a.vm.regs[rb];
                } else {
                    a.vm.regs[ra] %= a.vm.regs[rb];
                }
            }
            op::AND => a.vm.regs[ra] &= a.vm.regs[rb],
            op::OR => a.vm.regs[ra] |= a.vm.regs[rb],
            op::XOR => a.vm.regs[ra] ^= a.vm.regs[rb],
            op::SHL => a.vm.regs[ra] <<= a.vm.regs[rb] & 15,
            op::SHR => a.vm.regs[ra] >>= a.vm.regs[rb] & 15,
            op::EQ => a.vm.regs[ra] = (a.vm.regs[ra] == a.vm.regs[rb]) as u16,
            op::LTU => a.vm.regs[ra] = (a.vm.regs[ra] < a.vm.regs[rb]) as u16,
            op::LTS => a.vm.regs[ra] = ((a.vm.regs[ra] as i16) < (a.vm.regs[rb] as i16)) as u16,
            op::BRNZ | op::JUMP => {
                if opc == op::JUMP || a.vm.regs[ra] != 0 {
                    let target = a.vm.pc as i32 + 1 + imm as i16 as i32;
                    if target >= 0 && target < m.len as i32 {
                        next = target as u16;
                    }
                }
            }
            op::CALL => match a.bind(imm) {
                None => a.vm.fault = Fault::MissingTag as u8,
                Some(_) if a.vm.stack.len() >= MAX_FRAMES => a.vm.fault = Fault::StackFull as u8,
                Some(slot) => {
                    a.vm.stack.push((a.vm.module, a.vm.pc + 1));
                    a.vm.module = slot;
                    a.vm.pc = 0;
                    continue;
                }
            },
            op::RET => {
                if let Some((mi, pc)) = a.vm.stack.pop() {
                    a.vm.module = mi;
                    a.vm.pc = pc;
                    continue;
                }
                a.vm.module = entry;
                a.vm.pc = 0;
                break;
            }
            op::LOAD => {
                let addr = a.vm.regs[rb] as usize;
                if addr < 64 {
                    a.vm.regs[ra] = a.mem[addr];
                } else {
                    a.vm.regs[ra] = 0;
                    a.vm.fault = Fault::BadAddress as u8;
                }
            }
            op::STORE => {
                let addr = a.vm.regs[rb] as usize;
                if addr < 64 {
                    a.mem[addr] = a.vm.regs[ra];
                } else {
                    a.vm.fault = Fault::BadAddress as u8;
                }
            }
            op::RANDOM => {
                let mut st = Stream::at(ctrl_key, s.tick, a.s.draws);
                a.vm.regs[ra] = st.next_u32() as u16;
                a.s.draws += 1;
            }
            op::YIELD => {
                a.vm.pc = next;
                break;
            }
            op::READ => {
                let domain = a.vm.regs[0];
                if imm != 0 {
                    a.vm.regs[ra] = 0;
                    a.vm.fault = Fault::BadAddress as u8;
                } else if dom::is_external(domain) {
                    if a.s.queries >= MAX_QUERIES {
                        a.vm.regs[ra] = 0;
                        a.vm.fault = Fault::QueryLimit as u8;
                    } else if a.energy < QUERY_COST {
                        a.vm.fault = Fault::EnergyStop as u8;
                        break;
                    } else {
                        pay(a, QUERY_COST);
                        a.s.queries += 1;
                        let v = read(a, s);
                        set_read(a, ra, v);
                    }
                } else {
                    let v = read(a, s);
                    set_read(a, ra, v);
                }
            }
            op::WRITE | op::DRIVE | op::LINK | op::UNLINK | op::TRANSFER | op::WORK | op::EMIT
            | op::REPRO | op::COPY => {
                // an external source record costs one sensing query (SPEC §20b)
                if opc == op::COPY && s.m1 && unpack(a.vm.regs[0]).map(|x| x.0) == Some(dom::PUBLIC_TAPE) {
                    if a.s.queries >= MAX_QUERIES {
                        a.vm.fault = Fault::QueryLimit as u8;
                        a.vm.pc = next;
                        break;
                    }
                    if a.energy < QUERY_COST {
                        a.vm.fault = Fault::EnergyStop as u8;
                        break;
                    }
                    pay(a, QUERY_COST);
                    a.s.queries += 1;
                }
                let cost = attempt_cost(opc, imm);
                a.vm.pc = next;
                if a.energy < cost {
                    a.receipt = Some((Outcome::Energy as u8, 0));
                } else {
                    pay(a, cost);
                    let mut r8 = [0u16; 8];
                    r8.copy_from_slice(&a.vm.regs[..8]);
                    let copy = if opc == op::COPY { capture_copy(a, s) } else { None };
                    let target_uid = if opc == op::TRANSFER { capture_target(a, s) } else { None };
                    let parcel_uid = if s.parcels { capture_parcel(a, s, opc) } else { None };
                    a.s.intent = Some(Intent { opc, sub: imm, r: r8, copy, target_uid, parcel_uid });
                }
                break;
            }
            _ => a.vm.fault = Fault::InvalidOpcode as u8,
        }
        a.vm.pc = next;
    }
}

fn set_read(a: &mut Agent, ra: usize, v: Result<u16, Fault>) {
    match v {
        Ok(x) => a.vm.regs[ra] = x,
        Err(f) => {
            a.vm.regs[ra] = 0;
            a.vm.fault = f as u8;
        }
    }
}
