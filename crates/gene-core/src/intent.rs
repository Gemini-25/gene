//! Effect intents and outcomes. SPEC §10.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outcome {
    Ok = 0,
    Partial = 1,
    Noop = 2,
    Boundary = 3,
    Blocked = 4,
    Energy = 5,
    Matter = 6,
    Capacity = 7,
    InvalidTarget = 8,
    Stale = 9,
    Limits = 10,
    Format = 11,
    Occluded = 12,
    QueryLimit = 13,
    Busy = 14,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fault {
    Ok = 0,
    BadAddress = 1,
    DivZero = 2,
    InvalidOpcode = 3,
    StackFull = 4,
    QueryLimit = 5,
    MissingTag = 6,
    Occluded = 7,
    Boundary = 8,
    EnergyStop = 9,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CopyCapture {
    /// Tile of an external (public) source record.
    pub src_tile: Option<u32>,
    pub src_uid: u64,
    pub src_rev: u64,
    /// Bytes readable at Think time (prefix up to the first hole / logical end).
    pub bytes: Vec<u16>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Intent {
    pub opc: u8,
    pub sub: u16,
    pub r: [u16; 8],
    pub copy: Option<CopyCapture>,
    /// Occupant uid captured at Think for ActorInventory destinations.
    pub target_uid: Option<u64>,
}

pub mod sub {
    pub const DRIVE_TRANSLATE: u16 = 0;
    pub const DRIVE_ROTATE: u16 = 1;
    pub const WRITE_CELL: u16 = 0;
    pub const WRITE_RESIZE: u16 = 1;
    pub const WRITE_ORIENT: u16 = 4;
    pub const WRITE_PORT: u16 = 5;
    pub const WORK_COMPRESS: u16 = 0;
    pub const WORK_ABRADE: u16 = 1;
    pub const WORK_HEAT: u16 = 2;
    pub const REPRO_ALLOCATE: u16 = 0;
    pub const REPRO_RESIZE: u16 = 1;
    pub const REPRO_FINISH: u16 = 2;
    pub const REPRO_STEP: u16 = 3;
    pub const REPRO_REOPEN: u16 = 4;
    pub const REPRO_RELEASE: u16 = 5;
    pub const REPRO_CANCEL: u16 = 6;
}

pub mod dom {
    pub const SELF: u16 = 0;
    pub const LOOSE: u16 = 1;
    pub const PARCEL: u16 = 2;
    pub const ACTOR: u16 = 3;
    pub const SIGNAL: u16 = 4;
    pub const PUBLIC_TAPE: u16 = 5;
    pub const OWN_CODE: u16 = 6;
    pub const OWN_DRAFT: u16 = 7;
    pub const EMBRYO: u16 = 8;
    pub const LOCAL_TRACE: u16 = 9;
    pub const PORE: u16 = 10;
    pub const SURFACE: u16 = 11;

    pub fn is_external(d: u16) -> bool {
        matches!(d, 1 | 2 | 3 | 4 | 5 | 9 | 10 | 11)
    }
}

/// Attempt cost reserved when an effect is submitted.
pub fn attempt_cost(opc: u8, sub_op: u16) -> u32 {
    use crate::isa::op;
    match (opc, sub_op) {
        (op::REPRO, sub::REPRO_ALLOCATE | sub::REPRO_FINISH | sub::REPRO_RELEASE) => 256,
        (op::EMIT, _) => 128,
        _ => 64,
    }
}

/// Packed local address: bits 0..3 domain, 4..6 offset, rest zero.
pub fn unpack(addr: u16) -> Option<(u16, u8)> {
    if addr >> 7 != 0 {
        return None;
    }
    let off = ((addr >> 4) & 7) as u8;
    if off > 4 {
        return None;
    }
    Some((addr & 15, off))
}

/// Relative (dx, dy) for packed offsets 0 self, 1 front, 2 right, 3 back, 4 left.
pub const OFFSETS: [(i32, i32); 5] = [(0, 0), (0, -1), (1, 0), (0, 1), (-1, 0)];

/// Rotate a relative vector by orientation (clockwise quarter turns).
#[inline]
pub fn rotate(orient: u8, dx: i32, dy: i32) -> (i32, i32) {
    match orient & 3 {
        0 => (dx, dy),
        1 => (-dy, dx),
        2 => (-dx, -dy),
        _ => (dy, -dx),
    }
}
