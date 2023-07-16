pub struct CPU {
    pub memory: [i8; 0xFFFF],
    pub registers: Registers,
    pub stack: [i8; 0xC],
    pub instrucion_pointer: i16,
    pub timers: Timers
}

pub struct Registers {
    pub general: [i8; 0xF],
    pub i: i8
}

pub struct Timers {
    pub delay: i8,
    pub sound: i8
}