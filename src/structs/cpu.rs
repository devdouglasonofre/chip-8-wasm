pub struct CPU {
    pub memory: [u8; 0xFFFF],
    pub registers: Registers,
    pub stack: [u8; 0xC],
    pub instrucion_pointer: i16,
    pub timers: Timers
}

pub struct Registers {
    pub general: [u8; 0xF],
    pub i: u8
}

pub struct Timers {
    pub delay: u8,
    pub sound: u8
}