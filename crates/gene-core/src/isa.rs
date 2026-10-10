//! Instruction encoding and a small assembler/disassembler. SPEC §8.

use crate::genome::{Module, Program};
use std::collections::BTreeMap;

pub mod op {
    pub const NOP: u8 = 0x00;
    pub const CONST: u8 = 0x01;
    pub const MOV: u8 = 0x02;
    pub const ADD: u8 = 0x03;
    pub const SUB: u8 = 0x04;
    pub const MUL: u8 = 0x05;
    pub const DIV: u8 = 0x06;
    pub const REM: u8 = 0x07;
    pub const AND: u8 = 0x08;
    pub const OR: u8 = 0x09;
    pub const XOR: u8 = 0x0A;
    pub const SHL: u8 = 0x0B;
    pub const SHR: u8 = 0x0C;
    pub const EQ: u8 = 0x0D;
    pub const LTU: u8 = 0x0E;
    pub const LTS: u8 = 0x0F;
    pub const BRNZ: u8 = 0x10;
    pub const JUMP: u8 = 0x11;
    pub const CALL: u8 = 0x12;
    pub const RET: u8 = 0x13;
    pub const LOAD: u8 = 0x14;
    pub const STORE: u8 = 0x15;
    pub const RANDOM: u8 = 0x16;
    pub const YIELD: u8 = 0x17;
    pub const READ: u8 = 0x20;
    pub const WRITE: u8 = 0x21;
    pub const DRIVE: u8 = 0x22;
    pub const LINK: u8 = 0x23;
    pub const UNLINK: u8 = 0x24;
    pub const TRANSFER: u8 = 0x25;
    pub const WORK: u8 = 0x26;
    pub const EMIT: u8 = 0x27;
    pub const REPRO: u8 = 0x28;
    pub const COPY: u8 = 0x29;
}

#[inline]
pub fn encode(opc: u8, a: u8, b: u8, imm: u16) -> u32 {
    opc as u32 | ((a as u32 & 15) << 8) | ((b as u32 & 15) << 12) | (imm as u32) << 16
}

#[inline]
pub fn decode(w: u32) -> (u8, usize, usize, u16) {
    (w as u8, ((w >> 8) & 15) as usize, ((w >> 12) & 15) as usize, (w >> 16) as u16)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    None,
    Ra,
    RaRb,
    RaImm,
    RaLabel,
    Label,
    Imm,
}

const TABLE: &[(&str, u8, Shape)] = &[
    ("NOP", op::NOP, Shape::None),
    ("CONST", op::CONST, Shape::RaImm),
    ("MOV", op::MOV, Shape::RaRb),
    ("ADD", op::ADD, Shape::RaRb),
    ("SUB", op::SUB, Shape::RaRb),
    ("MUL", op::MUL, Shape::RaRb),
    ("DIV", op::DIV, Shape::RaRb),
    ("REM", op::REM, Shape::RaRb),
    ("AND", op::AND, Shape::RaRb),
    ("OR", op::OR, Shape::RaRb),
    ("XOR", op::XOR, Shape::RaRb),
    ("SHL", op::SHL, Shape::RaRb),
    ("SHR", op::SHR, Shape::RaRb),
    ("EQ", op::EQ, Shape::RaRb),
    ("LTU", op::LTU, Shape::RaRb),
    ("LTS", op::LTS, Shape::RaRb),
    ("BRNZ", op::BRNZ, Shape::RaLabel),
    ("JUMP", op::JUMP, Shape::Label),
    ("CALL", op::CALL, Shape::Imm),
    ("RET", op::RET, Shape::None),
    ("LOAD", op::LOAD, Shape::RaRb),
    ("STORE", op::STORE, Shape::RaRb),
    ("RANDOM", op::RANDOM, Shape::Ra),
    ("YIELD", op::YIELD, Shape::None),
    ("READ", op::READ, Shape::Ra),
    ("WRITE", op::WRITE, Shape::Imm),
    ("DRIVE", op::DRIVE, Shape::Imm),
    ("LINK", op::LINK, Shape::Imm),
    ("UNLINK", op::UNLINK, Shape::Imm),
    ("TRANSFER", op::TRANSFER, Shape::Imm),
    ("WORK", op::WORK, Shape::Imm),
    ("EMIT", op::EMIT, Shape::Imm),
    ("REPRO", op::REPRO, Shape::Imm),
    ("COPY", op::COPY, Shape::Imm),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsmError {
    pub line: usize,
    pub msg: String,
}

impl std::fmt::Display for AsmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}: {}", self.line, self.msg)
    }
}

enum Item {
    Word(u32),
    Branch { opc: u8, a: u8, label: String, line: usize },
}

struct ModBuild {
    tag: u16,
    items: Vec<Item>,
    labels: BTreeMap<String, usize>,
}

fn err(line: usize, msg: impl Into<String>) -> AsmError {
    AsmError { line, msg: msg.into() }
}

fn parse_num(s: &str, equ: &BTreeMap<String, i64>) -> Option<i64> {
    let s = s.trim();
    if let Some(v) = equ.get(s) {
        return Some(*v);
    }
    let (neg, body) = match s.strip_prefix('-') {
        Some(b) => (true, b),
        None => (false, s),
    };
    let v = if let Some(h) = body.strip_prefix("0x").or_else(|| body.strip_prefix("0X")) {
        i64::from_str_radix(&h.replace('_', ""), 16).ok()?
    } else {
        body.replace('_', "").parse::<i64>().ok()?
    };
    Some(if neg { -v } else { v })
}

fn parse_u16(s: &str, equ: &BTreeMap<String, i64>, line: usize) -> Result<u16, AsmError> {
    let v = parse_num(s, equ).ok_or_else(|| err(line, format!("bad number '{s}'")))?;
    if !(-32768..=65535).contains(&v) {
        return Err(err(line, format!("value {v} out of u16/i16 range")));
    }
    Ok(v as u16)
}

fn parse_reg(s: &str, line: usize) -> Result<u8, AsmError> {
    let s = s.trim();
    let n = s
        .strip_prefix('r')
        .or_else(|| s.strip_prefix('R'))
        .and_then(|n| n.parse::<u8>().ok())
        .filter(|n| *n < 16)
        .ok_or_else(|| err(line, format!("bad register '{s}'")))?;
    Ok(n)
}

/// Assemble source text into a Program. Labels are module-local.
pub fn assemble(src: &str) -> Result<Program, AsmError> {
    let mut equ: BTreeMap<String, i64> = BTreeMap::new();
    let mut entry = 0u16;
    let mut point = 64u16;
    let mut structural = 2048u16;
    let mut weights = [4u16, 2, 2, 2, 1, 1, 1, 1];
    let mut mods: Vec<ModBuild> = Vec::new();

    for (ln0, raw) in src.lines().enumerate() {
        let line = ln0 + 1;
        let text = raw.split(';').next().unwrap().trim();
        if text.is_empty() {
            continue;
        }
        let mut rest = text;
        // labels (possibly several) at line start
        while let Some(pos) = rest.find(':') {
            let (lab, after) = rest.split_at(pos);
            let lab = lab.trim();
            if lab.is_empty() || lab.contains(char::is_whitespace) {
                break;
            }
            let m = mods.last_mut().ok_or_else(|| err(line, "label before .module"))?;
            if m.labels.insert(lab.to_string(), m.items.len()).is_some() {
                return Err(err(line, format!("duplicate label '{lab}'")));
            }
            rest = after[1..].trim();
        }
        if rest.is_empty() {
            continue;
        }
        let (mn, args) = match rest.find(char::is_whitespace) {
            Some(p) => (&rest[..p], rest[p..].trim()),
            None => (rest, ""),
        };
        let args: Vec<&str> =
            if args.is_empty() { vec![] } else { args.split(',').map(|s| s.trim()).collect() };
        if let Some(dir) = mn.strip_prefix('.') {
            match dir {
                "equ" => {
                    let parts: Vec<&str> = rest[4..].split_whitespace().collect();
                    if parts.len() != 2 {
                        return Err(err(line, ".equ NAME VALUE"));
                    }
                    let v = parse_num(parts[1], &equ)
                        .ok_or_else(|| err(line, format!("bad value '{}'", parts[1])))?;
                    equ.insert(parts[0].to_string(), v);
                }
                "entry" => entry = parse_u16(args[0], &equ, line)?,
                "point" => point = parse_u16(args[0], &equ, line)?,
                "structural" => structural = parse_u16(args[0], &equ, line)?,
                "weights" => {
                    if args.len() != 8 {
                        return Err(err(line, ".weights needs 8 values"));
                    }
                    for (i, a) in args.iter().enumerate() {
                        weights[i] = parse_u16(a, &equ, line)?;
                    }
                }
                "module" => mods.push(ModBuild {
                    tag: parse_u16(args[0], &equ, line)?,
                    items: vec![],
                    labels: BTreeMap::new(),
                }),
                "word" => {
                    let v = parse_num(args[0], &equ).ok_or_else(|| err(line, "bad word"))?;
                    mods.last_mut()
                        .ok_or_else(|| err(line, ".word before .module"))?
                        .items
                        .push(Item::Word(v as u32));
                }
                _ => return Err(err(line, format!("unknown directive .{dir}"))),
            }
            continue;
        }
        let up = mn.to_ascii_uppercase();
        let &(_, opc, shape) = TABLE
            .iter()
            .find(|(n, _, _)| *n == up)
            .ok_or_else(|| err(line, format!("unknown mnemonic '{mn}'")))?;
        let need = match shape {
            Shape::None => 0,
            Shape::Ra | Shape::Label | Shape::Imm => 1,
            _ => 2,
        };
        // Effects may omit the sub-op (defaults to 0).
        let args = if shape == Shape::Imm && args.is_empty() { vec!["0"] } else { args };
        if args.len() != need {
            return Err(err(line, format!("{up} expects {need} operand(s)")));
        }
        let m = mods.last_mut().ok_or_else(|| err(line, "instruction before .module"))?;
        let item = match shape {
            Shape::None => Item::Word(encode(opc, 0, 0, 0)),
            Shape::Ra => Item::Word(encode(opc, parse_reg(args[0], line)?, 0, 0)),
            Shape::RaRb => Item::Word(encode(
                opc,
                parse_reg(args[0], line)?,
                parse_reg(args[1], line)?,
                0,
            )),
            Shape::RaImm => {
                Item::Word(encode(opc, parse_reg(args[0], line)?, 0, parse_u16(args[1], &equ, line)?))
            }
            Shape::Imm => Item::Word(encode(opc, 0, 0, parse_u16(args[0], &equ, line)?)),
            Shape::RaLabel => Item::Branch {
                opc,
                a: parse_reg(args[0], line)?,
                label: args[1].to_string(),
                line,
            },
            Shape::Label => Item::Branch { opc, a: 0, label: args[0].to_string(), line },
        };
        m.items.push(item);
    }

    if mods.is_empty() {
        return Err(err(0, "no modules"));
    }
    let mut modules = Vec::new();
    let mut words = Vec::new();
    for m in &mods {
        if m.items.is_empty() {
            return Err(err(0, format!("module {} is empty", m.tag)));
        }
        let start = words.len() as u16;
        for (pc, it) in m.items.iter().enumerate() {
            words.push(match it {
                Item::Word(w) => *w,
                Item::Branch { opc, a, label, line } => {
                    let target = *m
                        .labels
                        .get(label)
                        .ok_or_else(|| err(*line, format!("unknown label '{label}'")))?;
                    let rel = target as i64 - (pc as i64 + 1);
                    encode(*opc, *a, 0, rel as i16 as u16)
                }
            });
        }
        modules.push(Module { tag: m.tag, start, len: m.items.len() as u16 });
    }
    let p = Program {
        entry_tag: entry,
        point_threshold: point,
        structural_threshold: structural,
        weights,
        modules,
        words,
    };
    if p.module_index(entry).is_none() {
        return Err(err(0, format!("entry tag {entry} has no module")));
    }
    if !p.within_limits() {
        return Err(err(0, "program exceeds format limits"));
    }
    Ok(p)
}

pub fn mnemonic(opc: u8) -> Option<&'static str> {
    TABLE.iter().find(|(_, o, _)| *o == opc).map(|(n, _, _)| *n)
}

pub fn disassemble_word(w: u32) -> String {
    let (opc, a, b, imm) = decode(w);
    let Some(&(name, _, shape)) = TABLE.iter().find(|(_, o, _)| *o == opc) else {
        return format!(".word 0x{w:08x}");
    };
    match shape {
        Shape::None => name.to_string(),
        Shape::Ra => format!("{name} r{a}"),
        Shape::RaRb => format!("{name} r{a}, r{b}"),
        Shape::RaImm => format!("{name} r{a}, {imm}"),
        Shape::Imm => format!("{name} {imm}"),
        Shape::RaLabel => format!("{name} r{a}, {:+}", imm as i16),
        Shape::Label => format!("{name} {:+}", imm as i16),
    }
}

pub fn disassemble(p: &Program) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        ".entry {}\n.point {}\n.structural {}\n.weights {}\n",
        p.entry_tag,
        p.point_threshold,
        p.structural_threshold,
        p.weights.iter().map(|w| w.to_string()).collect::<Vec<_>>().join(",")
    ));
    for m in &p.modules {
        s.push_str(&format!(".module {}\n", m.tag));
        for pc in 0..m.len {
            let w = p.words[(m.start + pc) as usize];
            s.push_str(&format!("  {:4}: {}\n", pc, disassemble_word(w)));
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assembles_branches_and_equ() {
        let p = assemble(
            ".equ K 7\n.module 0\ntop: CONST r1, K\n BRNZ r1, top\n JUMP end\n NOP\nend: YIELD\n",
        )
        .unwrap();
        assert_eq!(p.words.len(), 5);
        assert_eq!(decode(p.words[0]), (op::CONST, 1, 0, 7));
        assert_eq!(decode(p.words[1]).3 as i16, -2);
        assert_eq!(decode(p.words[2]).3 as i16, 1);
        assert!(disassemble(&p).contains("BRNZ r1, -2"));
    }

    #[test]
    fn rejects_unknown_label() {
        assert!(assemble(".module 0\n JUMP nowhere\n").is_err());
    }
}
