struct CPURegisters {
    accumulator: u64,
    base: u64,
    counter: u64,
    stack_pointer: u64,
    stack_base_pointer: u64,
    destination: u64,
    source: u64,
    data: u64,
}

impl CPURegisters {
    fn new() -> CPURegisters {
        CPURegisters {
            accumulator: 0,
            base: 0,
            counter: 0,
            stack_pointer: 0,
            stack_base_pointer: 0,
            destination: 0,
            source: 0,
            data: 0,
        }
    }

    fn get_64(&self, register: RegisterIdentifier64Bit) -> u64 {
        match register {
            RegisterIdentifier64Bit::RAX => self.accumulator,
            RegisterIdentifier64Bit::RBX => self.base,
            RegisterIdentifier64Bit::RCX => self.counter,
            RegisterIdentifier64Bit::RSP => self.stack_pointer,
            RegisterIdentifier64Bit::RBP => self.stack_base_pointer,
            RegisterIdentifier64Bit::RDI => self.destination,
            RegisterIdentifier64Bit::RSI => self.source,
            RegisterIdentifier64Bit::RDX => self.data,
        }
    }

    fn get_32(&self, register: RegisterIdentifier32Bit) -> u32 {
        match register {
            RegisterIdentifier32Bit::EAX => (self.accumulator & 0xFFFFFFFF).try_into().unwrap(),
            RegisterIdentifier32Bit::EBX => (self.base & 0xFFFFFFFF).try_into().unwrap(),
            RegisterIdentifier32Bit::ECX => (self.counter & 0xFFFFFFFF).try_into().unwrap(),
            RegisterIdentifier32Bit::ESP => (self.stack_pointer & 0xFFFFFFFF).try_into().unwrap(),
            RegisterIdentifier32Bit::EBP => {
                (self.stack_base_pointer & 0xFFFFFFFF).try_into().unwrap()
            }
            RegisterIdentifier32Bit::EDI => (self.destination & 0xFFFFFFFF).try_into().unwrap(),
            RegisterIdentifier32Bit::ESI => (self.source & 0xFFFFFFFF).try_into().unwrap(),
            RegisterIdentifier32Bit::EDX => (self.data & 0xFFFFFFFF).try_into().unwrap(),
        }
    }

    fn get_16(&self, register: RegisterIdentifier16Bit) -> u16 {
        match register {
            RegisterIdentifier16Bit::AX => (self.accumulator & 0xFFFF).try_into().unwrap(),
            RegisterIdentifier16Bit::BX => (self.base & 0xFFFF).try_into().unwrap(),
            RegisterIdentifier16Bit::CX => (self.counter & 0xFFFF).try_into().unwrap(),
            RegisterIdentifier16Bit::SP => (self.stack_pointer & 0xFFFF).try_into().unwrap(),
            RegisterIdentifier16Bit::BP => (self.stack_base_pointer & 0xFFFF).try_into().unwrap(),
            RegisterIdentifier16Bit::DI => (self.destination & 0xFFFF).try_into().unwrap(),
            RegisterIdentifier16Bit::SI => (self.source & 0xFFFF).try_into().unwrap(),
            RegisterIdentifier16Bit::DX => (self.data & 0xFFFF).try_into().unwrap(),
        }
    }

    fn get_8(&self, register: RegisterIdentifier8Bit) -> u8 {
        match register {
            RegisterIdentifier8Bit::AH => ((self.accumulator >> 8) & 0xFF).try_into().unwrap(),
            RegisterIdentifier8Bit::AL => (self.accumulator & 0xFF).try_into().unwrap(),
            RegisterIdentifier8Bit::BH => ((self.base >> 8) & 0xFF).try_into().unwrap(),
            RegisterIdentifier8Bit::BL => (self.base & 0xFF).try_into().unwrap(),
            RegisterIdentifier8Bit::CH => ((self.counter >> 8) & 0xFF).try_into().unwrap(),
            RegisterIdentifier8Bit::CL => (self.counter & 0xFF).try_into().unwrap(),
            RegisterIdentifier8Bit::SPL => ((self.stack_pointer >> 8) & 0xFF).try_into().unwrap(),
            RegisterIdentifier8Bit::BPL => {
                ((self.stack_base_pointer >> 8) & 0xFF).try_into().unwrap()
            }
            RegisterIdentifier8Bit::DIL => ((self.destination >> 8) & 0xFF).try_into().unwrap(),
            RegisterIdentifier8Bit::SIL => ((self.source >> 8) & 0xFF).try_into().unwrap(),
            RegisterIdentifier8Bit::DH => ((self.data >> 8) & 0xFF).try_into().unwrap(),
            RegisterIdentifier8Bit::DL => (self.data & 0xFF).try_into().unwrap(),
        }
    }

    fn set_64(&mut self, register: RegisterIdentifier64Bit, value: u64) {
        match register {
            RegisterIdentifier64Bit::RAX => self.accumulator = value,
            RegisterIdentifier64Bit::RBX => self.base = value,
            RegisterIdentifier64Bit::RCX => self.counter = value,
            RegisterIdentifier64Bit::RSP => self.stack_pointer = value,
            RegisterIdentifier64Bit::RBP => self.stack_base_pointer = value,
            RegisterIdentifier64Bit::RDI => self.destination = value,
            RegisterIdentifier64Bit::RSI => self.source = value,
            RegisterIdentifier64Bit::RDX => self.data = value,
        }
    }

    fn set_32(&mut self, register: RegisterIdentifier32Bit, value: u32) {
        match register {
            RegisterIdentifier32Bit::EAX => {
                self.accumulator = (self.accumulator & 0xFFFFFFFF00000000) | value as u64
            }
            RegisterIdentifier32Bit::EBX => {
                self.base = (self.base & 0xFFFFFFFF00000000) | value as u64
            }
            RegisterIdentifier32Bit::ECX => {
                self.counter = (self.counter & 0xFFFFFFFF00000000) | value as u64
            }
            RegisterIdentifier32Bit::ESP => {
                self.stack_pointer = (self.stack_pointer & 0xFFFFFFFF00000000) | value as u64
            }
            RegisterIdentifier32Bit::EBP => {
                self.stack_base_pointer =
                    (self.stack_base_pointer & 0xFFFFFFFF00000000) | value as u64
            }
            RegisterIdentifier32Bit::EDI => {
                self.destination = (self.destination & 0xFFFFFFFF00000000) | value as u64
            }
            RegisterIdentifier32Bit::ESI => {
                self.source = (self.source & 0xFFFFFFFF00000000) | value as u64
            }
            RegisterIdentifier32Bit::EDX => {
                self.data = (self.data & 0xFFFFFFFF00000000) | value as u64
            }
        }
    }

    fn set_16(&mut self, register: RegisterIdentifier16Bit, value: u16) {
        match register {
            RegisterIdentifier16Bit::AX => {
                self.accumulator = (self.accumulator & 0xFFFFFFFFFFFF0000) | value as u64
            }
            RegisterIdentifier16Bit::BX => {
                self.base = (self.base & 0xFFFFFFFFFFFF0000) | value as u64
            }
            RegisterIdentifier16Bit::CX => {
                self.counter = (self.counter & 0xFFFFFFFFFFFF0000) | value as u64
            }
            RegisterIdentifier16Bit::SP => {
                self.stack_pointer = (self.stack_pointer & 0xFFFFFFFFFFFF0000) | value as u64
            }
            RegisterIdentifier16Bit::BP => {
                self.stack_base_pointer =
                    (self.stack_base_pointer & 0xFFFFFFFFFFFF0000) | value as u64
            }
            RegisterIdentifier16Bit::DI => {
                self.destination = (self.destination & 0xFFFFFFFFFFFF0000) | value as u64
            }
            RegisterIdentifier16Bit::SI => {
                self.source = (self.source & 0xFFFFFFFFFFFF0000) | value as u64
            }
            RegisterIdentifier16Bit::DX => {
                self.data = (self.data & 0xFFFFFFFFFFFF0000) | value as u64
            }
        }
    }

    fn set_8(&mut self, register: RegisterIdentifier8Bit, value: u8) {
        match register {
            RegisterIdentifier8Bit::AH => {
                self.accumulator = (self.accumulator & 0xFFFFFFFFFFFF00FF) | (value as u64) << 8
            }
            RegisterIdentifier8Bit::AL => {
                self.accumulator = (self.accumulator & 0xFFFFFFFFFFFFFF00) | value as u64
            }
            RegisterIdentifier8Bit::BH => {
                self.base = (self.base & 0xFFFFFFFFFFFF00FF) | (value as u64) << 8
            }
            RegisterIdentifier8Bit::BL => {
                self.base = (self.base & 0xFFFFFFFFFFFFFF00) | value as u64
            }
            RegisterIdentifier8Bit::CH => {
                self.counter = (self.counter & 0xFFFFFFFFFFFF00FF) | (value as u64) << 8
            }
            RegisterIdentifier8Bit::CL => {
                self.counter = (self.counter & 0xFFFFFFFFFFFFFF00) | value as u64
            }
            RegisterIdentifier8Bit::SPL => {
                self.stack_pointer = (self.stack_pointer & 0xFFFFFFFFFFFFFF00) | value as u64
            }
            RegisterIdentifier8Bit::BPL => {
                self.stack_base_pointer =
                    (self.stack_base_pointer & 0xFFFFFFFFFFFFFF00) | value as u64
            }
            RegisterIdentifier8Bit::DIL => {
                self.destination = (self.destination & 0xFFFFFFFFFFFFFF00) | value as u64
            }
            RegisterIdentifier8Bit::SIL => {
                self.source = (self.source & 0xFFFFFFFFFFFFFF00) | value as u64
            }
            RegisterIdentifier8Bit::DH => {
                self.data = (self.data & 0xFFFFFFFFFFFF00FF) | (value as u64) << 8
            }
            RegisterIdentifier8Bit::DL => {
                self.data = (self.data & 0xFFFFFFFFFFFFFF00) | value as u64
            }
        }
    }
}

enum RegisterIdentifier64Bit {
    RAX, //Accumulator
    RBX, //Base
    RCX, //Counter
    RSP, //Stack Pointer
    RBP, //Stack Base Pointer
    RDI, //Destination
    RSI, //Source
    RDX, //Data
}

enum RegisterIdentifier32Bit {
    EAX, //Accumulator
    EBX, //Base
    ECX, //Counter
    ESP, //Stack Pointer
    EBP, //Stack Base Pointer
    EDI, //Destination
    ESI, //Source
    EDX, //Data
}

enum RegisterIdentifier16Bit {
    AX, //Accumulator
    BX, //Base
    CX, //Counter
    SP, //Stack Pointer
    BP, //Stack Base Pointer
    DI, //Destination
    SI, //Source
    DX, //Data
}

enum RegisterIdentifier8Bit {
    AH, //Accumulator
    AL,
    BH, //Base
    BL,
    CH, //Counter
    CL,
    SPL, //Stack Pointer
    BPL, //Stack Base Pointer
    DIL, //Destination
    SIL, //Source
    DH,  //Data
    DL,
}

fn main() {
    println!("Hello, world!");
}
