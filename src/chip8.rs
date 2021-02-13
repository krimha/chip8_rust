
// TODO: Isn't keyboard supposed to be put anywhere in memory?
// TODO: Same "problem" with display


// CHIP-8 Machine state
//#[derive(Clone,Copy)]
#[derive(PartialEq,Eq,Debug,Clone,Copy)]
pub struct Machine {
    memory: [u8; 0x100],  // Main memory (4k bytes)
    v_reg: [u8; 16],      // Vx -registers (0...F)
    i_reg: u16,           // I-register used for memory addresses
    program_counter: u16, // Program counter. Points to current addresss    
    stack_pointer: usize,    // Stack pointer. Points to top of stack. Set to usize to allow it to index. *Can be* u8 according to docs

    stack: [u16; 16],
    keyboard: [bool; 16],   // Keys 0x0 - 0xF
    display: [u64; 32],             // 64x32 display. One element correcponds to one row.

    font: [u8; 16*5],       // Font sprites

    delay_timer_register: u8,
    sound_timer_register: u8,

    // TODO: Timers
}


impl Machine {
    // TODO: Write font sprites into correct memory locations
    

    // TODO: Derive/Implement Default instead?
    pub fn new() -> Machine {
        Machine {
            memory: [0; 0x100],
            v_reg: [0; 16],
            i_reg: 0,
            program_counter: 0x200,
            stack_pointer: 0,
            display: [0; 32],
            keyboard: [false; 16],
            stack: [0; 16],
            font : [0; 5*16],
            delay_timer_register: 0,
            sound_timer_register: 0,
        }
    }

    pub fn execute_instruction(&mut self, instruction: u16) {
        
        match instruction {
            0x00E0 =>  // CLS - Clear screen
                for val in self.display.iter_mut() { *val = 0; },
            0x00EE => { // RET - Return from subroutine             
                self.program_counter = self.stack[self.stack_pointer];
                self.stack_pointer -= 1;
            },

            // Pattern matching based on first digit                    
            _ => match instruction >> 12 { 
                0x0 => {}, // SYS - Do nothing (on modern systems)
                0x1 => self.program_counter = instruction & 0x0FFF, // JP - PC jump to address 
                0x2 => {
                    self.stack_push(self.program_counter);
                    self.program_counter = instruction & 0x0FFF;
                },
                0x3 => {
                    // Best way to "cast" to usize?
                    let reg_num : usize = ((instruction & 0x0F00) >> 8).into();
                    let val = instruction.to_be_bytes()[1];
                    if self.v_reg[reg_num] == val {
                        self.program_counter += 2;
                    }
                },
                0x4 => {
                    // Best way to "cast" to usize?
                    let reg_num : usize = ((instruction & 0x0F00) >> 8).into();
                    let val = instruction.to_be_bytes()[1];
                    if self.v_reg[reg_num] != val {
                        self.program_counter += 2;
                    }
                },
                0x5 => {
                    let reg1: usize = ((instruction & 0x0F00) >> 8).into();
                    let reg2: usize = ((instruction & 0x00F0) >> 4).into();
                    if self.v_reg[reg1] == self.v_reg[reg2] {
                        self.program_counter += 2;
                    }
                },
                0x6 => {
                    let reg: usize = ((instruction & 0x0F00) >> 8).into();
                    let val = instruction.to_be_bytes()[1];
                    self.v_reg[reg] = val;
                }
                0x7 => { // Add Vx, byte
                    let reg: usize = ((instruction & 0x0F00) >> 8).into();
                    let val = instruction.to_be_bytes()[1];
                    self.v_reg[reg] += val;
                },
                0x8 => match instruction & 0x000F {
                    0x0 => {// LD, Vx, Vy
                        let reg1: usize = ((instruction & 0x0F00) >> 8).into();
                        let reg2: usize = ((instruction & 0x00F0) >> 4).into();
                        self.v_reg[reg1] = self.v_reg[reg2];
                    },
                    0x1 => { // Vx <- Vx OR Vy 
                        let reg1: usize = ((instruction & 0x0F00) >> 8).into();
                        let reg2: usize = ((instruction & 0x00F0) >> 4).into();
                        self.v_reg[reg1] |= self.v_reg[reg2];
                    }, 
                    0x2 => { // Vx <- Vx AND Vy
                        let reg1: usize = ((instruction & 0x0F00) >> 8).into();
                        let reg2: usize = ((instruction & 0x00F0) >> 4).into();
                        self.v_reg[reg1] &= self.v_reg[reg2];
                    },
                    0x3 => {
                        let reg1: usize = ((instruction & 0x0F00) >> 8).into();
                        let reg2: usize = ((instruction & 0x00F0) >> 4).into();
                        self.v_reg[reg1] ^= self.v_reg[reg2];
                    },
                    0x4 => {
                        let reg1: usize = ((instruction & 0x0F00) >> 8).into();
                        let reg2: usize = ((instruction & 0x00F0) >> 4).into();
                        let val1 = self.v_reg[reg1];
                        let val2 = self.v_reg[reg2];
                        let (val, overflow) = val1.overflowing_add(val2);
                        self.v_reg[reg1] = val;
                        self.v_reg[0xF] = overflow as u8;
                    }
                    0x5 => {
                        let reg1: usize = ((instruction & 0x0F00) >> 8).into();
                        let reg2: usize = ((instruction & 0x00F0) >> 4).into();
                        let val1 = self.v_reg[reg1];
                        let val2 = self.v_reg[reg2];
                        let (val, underflow) = val1.overflowing_sub(val2);
                        self.v_reg[reg1] = val;
                        self.v_reg[0xF] = underflow as u8;
                    }
                    _ => {}
                },
                _ => {}
            },
        }
    }

    fn stack_peek(&self) -> u16 {
        self.stack[self.stack_pointer]
    }

    fn stack_push(&mut self, val: u16) {
        self.stack_pointer += 1;
        self.stack[self.stack_pointer] = val;   
    }




}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_new() {
        let machine = Machine::new();
        for val in machine.memory.iter() {
            assert!(*val == 0);
        }

        for val in machine.v_reg.iter() {
            assert!(*val == 0);
        }

        for val in machine.stack.iter() {
            assert!(*val == 0);
        }
        assert_eq!(machine.i_reg, 0);
        assert_eq!(machine.program_counter, 0x200);
        assert_eq!(machine.stack_pointer, 0);

        assert_eq!(machine.delay_timer_register, 0);
        assert_eq!(machine.sound_timer_register, 0);
    }


    #[test]
    fn test_machine_execute_sys() {
        // SYSY instructions should be ignored
        let instruction: u16 = 0x0ABC;
        let mut machine = Machine::new();
        machine.display[0] = 1;
        machine.memory[0] = 1;
        
        let machine_backup = machine.clone();
        machine.execute_instruction(instruction);
        assert_eq!(machine, machine_backup);
    }

    #[test]
    fn test_machine_execute_cls() {
        let instruction: u16 = 0x00E0;
        let mut machine = Machine::new();
        machine.display.fill(1 << 31);
        machine.execute_instruction(instruction);

        assert_eq!(machine.display, [0;32]);
    }

    #[test]
    fn test_machine_execute_ret() {
        let instruction = 0x00EE;
        let mut machine = Machine::new();

        machine.stack[1] = 0x001;
        machine.stack_pointer = 1;
        machine.program_counter = 0x000;

        machine.execute_instruction(instruction);

        assert_eq!(machine.stack_pointer, 0);
        assert_eq!(machine.program_counter, 0x001);
    }

    #[test]
    fn test_machine_execute_jp() {
        let instruction = 0x17AC;
        let mut machine = Machine::new();
        machine.execute_instruction(instruction);

        assert_eq!(machine.program_counter, 0x7AC);
    }

    #[test]
    fn test_machine_execute_call() {
        let instruction = 0x26C2;
        let mut machine = Machine::new();
        let pc_value = 0x678;
        machine.program_counter = pc_value;

        machine.execute_instruction(instruction);

        assert_eq!(machine.stack_pointer, 1);
        assert_eq!(machine.stack_peek(), pc_value);
        assert_eq!(machine.program_counter, 0x6C2);
    }

    #[test]
    fn test_machine_execute_se() {
        let mut machine = Machine::new();
        machine.execute_instruction(0x3000);
        assert_eq!(machine.program_counter, 0x202);

        machine = Machine::new();
        machine.execute_instruction(0x3023);
        assert_eq!(machine.program_counter, 0x200);

        machine = Machine::new();
        machine.v_reg[1] = 0x23;
        machine.execute_instruction(0x3123);
        assert_eq!(machine.program_counter, 0x202)
    }

    #[test]
    fn test_machine_execute_sne() {
        let mut machine = Machine::new();
        machine.execute_instruction(0x4000);
        assert_eq!(machine.program_counter, 0x200);

        machine = Machine::new();
        machine.execute_instruction(0x4023);
        assert_eq!(machine.program_counter, 0x202);

        machine = Machine::new();
        machine.v_reg[1] = 0x23;
        machine.execute_instruction(0x4123);
        assert_eq!(machine.program_counter, 0x200)
    }

    #[test]
    fn test_machine_exeute_se_reg() {
        let mut machine = Machine::new();
        machine.v_reg[0] = 5;
        machine.execute_instruction(0x5000);
        assert_eq!(machine.program_counter, 0x202);

        machine = Machine::new();
        machine.v_reg[1] = 5;
        machine.execute_instruction(0x5010);
        assert_eq!(machine.program_counter, 0x200);
    }

    #[test]
    fn test_machine_execute_ld() {
        let mut machine = Machine::new();
        machine.execute_instruction(0x6023);
        assert_eq!(machine.v_reg[0], 0x23);

        machine.execute_instruction(0x6001);
        assert_eq!(machine.v_reg[0], 0x1);

        machine.execute_instruction(0x6123);
        assert_eq!(machine.v_reg[1], 0x23);
    }

    #[test]
    fn test_mahine_execute_add() {
        let mut m = Machine::new();
        m.execute_instruction(0x7001);
        assert_eq!(m.v_reg[0], 1);

        m.execute_instruction(0x7112);
        assert_eq!(m.v_reg[1], 0x12);

        m.v_reg[2] = 0x1;
        m.execute_instruction(0x7212);
        assert_eq!(m.v_reg[2], 0x13);
    }

    #[test]
    fn test_machine_execute_ld_reg() {
        let mut m = Machine::new();

        m.execute_instruction(0x8010);
        assert_eq!(m.v_reg[0], m.v_reg[1]);

        m.v_reg[2] = 4;
        m.v_reg[3] = 6;
        m.execute_instruction(0x8230);
        assert_eq!(m.v_reg[2], 6);

        m.v_reg[4] = 8;
        m.v_reg[5] = 10;
        m.execute_instruction(0x8450);
        assert_eq!(m.v_reg[4], 10);
    }

    #[test]
    fn test_machine_execute_or() {
        let mut m = Machine::new();
        m.v_reg[0] = 0x00000001;
        m.execute_instruction(0x8011);
        assert_eq!(m.v_reg[0], 0x1);

        m.v_reg[2] = 0b10111010;
        m.v_reg[3] = 0b01000101;
        m.execute_instruction(0x8231);
        assert_eq!(m.v_reg[2], 0xFF);
        assert_eq!(m.v_reg[3], 0b01000101);
    }

    #[test]
    fn test_machine_execute_and() {
        let mut m = Machine::new();
        m.v_reg[0] = 0x00000001;
        m.execute_instruction(0x8012);
        assert_eq!(m.v_reg[0], 0x0);

        m.v_reg[2] = 0b10111010;
        m.v_reg[3] = 0b01000101;
        m.execute_instruction(0x8232);
        assert_eq!(m.v_reg[2], 0x0);
        assert_eq!(m.v_reg[3], 0b01000101);

        m.v_reg[4] = 0b10111010;
        m.v_reg[5] = 0b01011101;
        m.execute_instruction(0x8452);
        assert_eq!(m.v_reg[4], 0b00011000);
        assert_eq!(m.v_reg[5], 0b01011101);
    }
    //Warning: Tests have been copied form the above test
    #[test]
    fn test_machine_execute_xor() {
        let mut m = Machine::new();
        m.v_reg[0] = 0x00000001;
        m.execute_instruction(0x8013);
        assert_eq!(m.v_reg[0], 0x1);

        m.v_reg[2] = 0b10111010;
        m.v_reg[3] = 0b01000101;
        m.execute_instruction(0x8233);
        assert_eq!(m.v_reg[2], 0xFF);
        assert_eq!(m.v_reg[3], 0b01000101);

        m.v_reg[4] = 0b10111010;
        m.v_reg[5] = 0b01011101;
        m.execute_instruction(0x8453);
        assert_eq!(m.v_reg[4], 0b11100111);
        assert_eq!(m.v_reg[5], 0b01011101);
    }

    #[test]
    fn test_machine_execute_add() {
        let mut m = Machine::new();
        m.v_reg[0] = 1;
        m.execute_instruction(0x8014);
        assert_eq!(m.v_reg[0], 1);

        m.v_reg[2] = 23;
        m.v_reg[3] = 20;
        m.execute_instruction(0x8234);
        assert_eq!(m.v_reg[2], 43);
        assert_eq!(m.v_reg[3], 20);
        assert_eq!(m.v_reg[0xF], 0x0);

        m.v_reg[4] = 200;
        m.v_reg[5] = 200;

        m.execute_instruction(0x8454);
        assert_eq!(m.v_reg[4], 0b10010000);
        assert_eq!(m.v_reg[0xF], 0x1);
    }

    #[test]
    fn test_machine_execute_sub() {
        let mut m = Machine::new();
        m.v_reg[0] = 1;
        m.execute_instruction(0x8015);
        assert_eq!(m.v_reg[0], 1);
        assert_eq!(m.v_reg[0xF], 0);

        m.v_reg[2] = 23;
        m.v_reg[3] = 20;
        m.execute_instruction(0x8235);
        assert_eq!(m.v_reg[2], 3);
        assert_eq!(m.v_reg[3], 20);
        assert_eq!(m.v_reg[0xF], 0x0);

        m.v_reg[4] = 0;
        m.v_reg[5] = 1;
        m.execute_instruction(0x8455);
        assert_eq!(m.v_reg[4], 0xFF);
        assert_eq!(m.v_reg[0xF], 0x1);
    }
}