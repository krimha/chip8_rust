use rand::Rng;

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

    font: [u16; 16],       // memory addresses to sprites

    delay_timer_register: u8,
    sound_timer_register: u8,

    // TODO: Timers
}


impl Machine {
    // TODO: Write font sprites into correct memory locations
    

    // TODO: Derive/Implement Default instead?
    pub fn new() -> Machine {
        let mut m = Machine {
            memory: [0; 0x100],
            v_reg: [0; 16],
            i_reg: 0,
            program_counter: 0x200,
            stack_pointer: 0,
            display: [0; 32],
            keyboard: [false; 16],
            stack: [0; 16],
            font : [ 0x000, 0x005, 0x00a, 0x00f, 0x014, 0x019, 0x01e, 0x023,
                0x028, 0x02d, 0x032, 0x037, 0x03c, 0x041, 0x046, 0x04b, ],
            delay_timer_register: 0,
            sound_timer_register: 0,
        };

        m.memory[0x000] = 0b11110000;
        m.memory[0x001] = 0b10010000;
        m.memory[0x002] = 0b10010000;
        m.memory[0x003] = 0b10010000;
        m.memory[0x004] = 0b11110000;

        m.memory[0x005] = 0b00100000;
        m.memory[0x006] = 0b01100000;
        m.memory[0x007] = 0b00100000;
        m.memory[0x008] = 0b00100000;
        m.memory[0x009] = 0b01110000;

        m.memory[0x00a] = 0b11110000;
        m.memory[0x00b] = 0b00010000;
        m.memory[0x00c] = 0b11110000;
        m.memory[0x00d] = 0b10000000;
        m.memory[0x00e] = 0b11110000;

        m.memory[0x00f] = 0b11110000;
        m.memory[0x010] = 0b00010000;
        m.memory[0x011] = 0b11110000;
        m.memory[0x012] = 0b00010000;
        m.memory[0x013] = 0b11110000;

        m.memory[0x014] = 0b10010000;
        m.memory[0x015] = 0b10010000;
        m.memory[0x016] = 0b11110000;
        m.memory[0x017] = 0b00010000;
        m.memory[0x018] = 0b00010000;

        m.memory[0x019] = 0b11110000;
        m.memory[0x01a] = 0b10000000;
        m.memory[0x01b] = 0b11110000;
        m.memory[0x01c] = 0b00010000;
        m.memory[0x01d] = 0b11110000;

        m.memory[0x01e] = 0b11110000;
        m.memory[0x01f] = 0b10000000;
        m.memory[0x020] = 0b11110000;
        m.memory[0x021] = 0b10010000;
        m.memory[0x022] = 0b11110000;

        m.memory[0x023] = 0b11110000;
        m.memory[0x024] = 0b00010000;
        m.memory[0x025] = 0b00100000;
        m.memory[0x026] = 0b01000000;
        m.memory[0x027] = 0b01000000;

        m.memory[0x028] = 0b11110000;
        m.memory[0x029] = 0b10010000;
        m.memory[0x02a] = 0b11110000;
        m.memory[0x02b] = 0b10010000;
        m.memory[0x02c] = 0b11110000;

        m.memory[0x02d] = 0b11110000;
        m.memory[0x02e] = 0b10010000;
        m.memory[0x02f] = 0b11110000;
        m.memory[0x030] = 0b00010000;
        m.memory[0x031] = 0b11110000;

        m.memory[0x032] = 0b11110000;
        m.memory[0x033] = 0b10010000;
        m.memory[0x034] = 0b11110000;
        m.memory[0x035] = 0b10010000;
        m.memory[0x036] = 0b10010000;

        m.memory[0x037] = 0b11100000;
        m.memory[0x038] = 0b10010000;
        m.memory[0x039] = 0b11100000;
        m.memory[0x03a] = 0b10010000;
        m.memory[0x03b] = 0b11100000;

        m.memory[0x03c] = 0b11110000;
        m.memory[0x03d] = 0b10000000;
        m.memory[0x03e] = 0b10000000;
        m.memory[0x03f] = 0b10000000;
        m.memory[0x040] = 0b11110000;

        m.memory[0x041] = 0b11100000;
        m.memory[0x042] = 0b1001000;
        m.memory[0x043] = 0b10010000;
        m.memory[0x044] = 0b10010000;
        m.memory[0x045] = 0b11100000;

        m.memory[0x046] = 0b11110000;
        m.memory[0x047] = 0b10000000;
        m.memory[0x048] = 0b11110000;
        m.memory[0x049] = 0b10000000;
        m.memory[0x04a] = 0b11110000;

        m.memory[0x04b] = 0b11110000;
        m.memory[0x04c] = 0b10000000;
        m.memory[0x04d] = 0b11110000;
        m.memory[0x04e] = 0b10000000;
        m.memory[0x04f] = 0b10000000;

        return m;
    }

    pub fn execute_instruction(&mut self, instruction: u16) {
        /*
        instruction is a 16 bit integer, divided in to zero or more fields, 
        depending on the instruction. The fields range in sizes of one to three
        bytes. The following variables extracts the possible parts, to avoid 
        duplcate code
        */
        let x : usize = ((instruction & 0x0F00) >> 8).into(); // 0x0x00
        let y : usize = ((instruction & 0x00F0) >> 4).into(); // 0x00y0
        let valx = self.v_reg[x];
        let valy = self.v_reg[y];

        let kk = instruction.to_be_bytes()[1]; // 0x00kk
        let nnn = instruction & 0x0FFF;       // 0x0nnn
        let n: u8 = (instruction & 0x000F).to_be_bytes()[1];        // 0x000n
        
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
                0x1 => self.program_counter = nnn, // JP - PC jump to address 
                0x2 => {
                    self.stack_push(self.program_counter);
                    self.program_counter = nnn;
                },
                0x3 => { // SE Vx, byte - Skip if Vx matches byte
                    if valx == kk {
                        self.program_counter += 2;
                    }
                },
                0x4 => { // SNE Vx, byte
                    if valx != kk {
                        self.program_counter += 2;
                    }
                },
                0x5 => { // SE Vx, Vy
                    if valx == valy {
                        self.program_counter += 2;
                    }
                },
                0x6 => { // LD Vx, byte
                    self.v_reg[x] = kk;
                }
                0x7 => { // Add Vx, byte
                    self.v_reg[x] += kk;
                },
                0x8 => match instruction & 0x000F {
                    0x0 => {// LD, Vx, Vy
                        self.v_reg[x] = valy;
                    },
                    0x1 => { // Vx <- Vx OR Vy 
                        self.v_reg[x] |= valy;
                    }, 
                    0x2 => { // Vx <- Vx AND Vy
                        self.v_reg[x] &= valy;
                    },
                    0x3 => {
                        self.v_reg[x] ^= valy;
                    },
                    0x4 => {
                        let (val, overflow) = valx.overflowing_add(valy);
                        self.v_reg[x] = val;
                        self.v_reg[0xF] = overflow as u8;
                    }
                    0x5 => {
                        let (val, underflow) = valx.overflowing_sub(valy);
                        self.v_reg[x] = val;
                        self.v_reg[0xF] = underflow as u8;
                    },
                    0x6 => {  // SHR  // TODO: Rewrite to use overflowing_shr?             
                        self.v_reg[0xF] = valx & 0x01;
                        self.v_reg[x] >>= 1;
                    },
                    0x7 => {
                        let (val, underflow) = valy.overflowing_sub(valx);
                        self.v_reg[x] = val;
                        self.v_reg[0xF] = (!underflow) as u8;
                    },
                    0xE => {
                        self.v_reg[0xF] = self.v_reg[x] >> 7;
                        self.v_reg[x] <<= 1;
                    }
                    _ => {}
                },
                0x9 => { // SNE
                    if valx != valy {
                        self.program_counter += 2;
                    }
                },
                0xA => { // LD i register
                    self.i_reg = nnn;
                },
                0xB => { // JP + V0                                                                       
                    self.program_counter = nnn + (self.v_reg[0] as u16);
                }
                // TODO: Write tests
                0xC => { // RND Vx AND kk
                    let r: u8 = rand::thread_rng().gen();
                    self.v_reg[x] = r & kk;
                }
                0xD => { // DRW Vx Vy n                       
                    let mut collision = 0;
                    for i  in 0..n{
                        let curr_display = self.display[((i + valy) % 32) as usize];

                        let mem_loc: usize = (self.i_reg + i as u16).into();
                        let mut row = (self.memory[mem_loc] as u64) << 7*8;
                        row = row.rotate_right(valx as u32); 

                        // Result if we did not care about collisions
                        let ored_result= curr_display | row;
                        let xored_result = curr_display ^ row;
                        collision |= (ored_result != xored_result) as u8;
                        self.display[((i + valy) % 32) as usize] = xored_result;
                    }
                    self.v_reg[0xF] = collision;
                }
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
        // Temporarily removed due to hardcoded sprites in memory.
        //for val in machine.memory.iter() {
        //    assert!(*val == 0);
        //}

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

    #[test]
    fn test_machine_excute_shr() {
        let mut m = Machine::new();
        m.v_reg[0] = 4;
        m.v_reg[1] = 5;
        m.execute_instruction(0x8016);
        assert_eq!(m.v_reg[0], 2);            
        assert_eq!(m.v_reg[0xF], 0);

        m.execute_instruction(0x8116);
        assert_eq!(m.v_reg[1], 2);
        assert_eq!(m.v_reg[0xF], 1);
    }

    #[test]
    fn test_machine_execute_subn() {
        let mut m = Machine::new();

        m.v_reg[0] = 2;
        m.v_reg[1] = 3;
        m.execute_instruction(0x8017);
        assert_eq!(m.v_reg[0], 1);
        assert_eq!(m.v_reg[0xF], 1);

        m.v_reg[2] = 3;
        m.v_reg[3] = 2;
        m.execute_instruction(0x8237);
        assert_eq!(m.v_reg[2], 0xFF);
        assert_eq!(m.v_reg[0xF], 0);
    }

    #[test]
    fn test_machine_execute_shl() {
        let mut m = Machine::new();
        m.v_reg[0] = 0x80;
        m.execute_instruction(0x800E);
        assert_eq!(m.v_reg[0], 0x00);
        assert_eq!(m.v_reg[0xF], 1);

        m.v_reg[1] = 0x8;
        m.execute_instruction(0x811E);
        assert_eq!(m.v_reg[1], 0x10);
        assert_eq!(m.v_reg[0xF], 0);
    }

    #[test]
    fn testmachine_execute_sne() {
        let mut m = Machine::new();        
        
        m.v_reg[0] = 2;
        m.execute_instruction(0x9010);
        assert_eq!(m.program_counter, 0x202);

        m = Machine::new();
        m.execute_instruction(0x9000);
        assert_eq!(m.program_counter, 0x200);
    }

    #[test]
    fn test_machine_execute_ld_i() {
        let mut m = Machine::new();
        
        m.execute_instruction(0xA456);
        assert_eq!(m.i_reg, 0x456);

        m.execute_instruction(0xAFFF);
        assert_eq!(m.i_reg, 0xFFF);
    }

    #[test]
    fn test_machine_execute_jp_pc() {
        let mut m = Machine::new();

        m.v_reg[0] = 1;
        m.execute_instruction(0xB211);
        assert_eq!(m.program_counter, 0x212);
    }

    #[test]
    fn test_machine_execute_drw() {
        let mut m = Machine::new();
        m.i_reg = m.font[0];
        m.v_reg[0] = 0;
        m.execute_instruction(0xD005);
        assert_eq!(m.display[0], 0xF000000000000000);
        assert_eq!(m.display[1], 0x9000000000000000);
        assert_eq!(m.display[2], 0x9000000000000000);
        assert_eq!(m.display[3], 0x9000000000000000);
        assert_eq!(m.display[4], 0xF000000000000000);
        assert_eq!(m.display[0xF], 0);
        assert_eq!(m.v_reg[0xF], 0);

        // Test different sprite
        let mut m = Machine::new();
        m.i_reg = m.font[1];
        m.v_reg[0] = 0;
        m.execute_instruction(0xD005);
        assert_eq!(m.display[0], 0x2000000000000000);
        assert_eq!(m.display[1], 0x6000000000000000);
        assert_eq!(m.display[2], 0x2000000000000000);
        assert_eq!(m.display[3], 0x2000000000000000);
        assert_eq!(m.display[4], 0x7000000000000000);
        
        // Display sprite at (1, 0);
        let mut m = Machine::new();
        m.i_reg = m.font[1];
        m.v_reg[0] = 0;
        m.v_reg[1] = 1;
        m.execute_instruction(0xD105);
        assert_eq!(m.display[0], 0x1000000000000000);
        assert_eq!(m.display[1], 0x3000000000000000);
        assert_eq!(m.display[2], 0x1000000000000000);
        assert_eq!(m.display[3], 0x1000000000000000);
        assert_eq!(m.display[4], 0x3800000000000000);
        
        // Display sprite at (1, 1);
        let mut m = Machine::new();
        m.i_reg = m.font[1];
        m.v_reg[0] = 1;
        m.execute_instruction(0xD005);
        assert_eq!(m.display[0], 0x0);
        assert_eq!(m.display[1], 0x1000000000000000);
        assert_eq!(m.display[2], 0x3000000000000000);
        assert_eq!(m.display[3], 0x1000000000000000);
        assert_eq!(m.display[4], 0x1000000000000000);
        assert_eq!(m.display[5], 0x3800000000000000);
        assert_eq!(m.v_reg[0xF], 0);

        // Overflow in y-direction
        let mut m = Machine::new();
        m.v_reg[0] = 0;
        m.i_reg = m.font[1];
        m.execute_instruction(0xD005);
        assert_eq!(m.display[0], 0x2000000000000000);
        assert_eq!(m.display[1], 0x6000000000000000);
        assert_eq!(m.display[2], 0x2000000000000000);
        assert_eq!(m.display[3], 0x2000000000000000);
        assert_eq!(m.display[4], 0x7000000000000000);
        assert_eq!(m.v_reg[0xF], 0);

        let mut m = Machine::new();
        m.v_reg[0] = 0;
        m.v_reg[1] = 29;
        m.i_reg = m.font[1];
        m.execute_instruction(0xD015);
        assert_eq!(m.display[29], 0x2000000000000000);
        assert_eq!(m.display[30], 0x6000000000000000);
        assert_eq!(m.display[31], 0x2000000000000000);
        assert_eq!(m.display[0], 0x2000000000000000);
        assert_eq!(m.display[1], 0x7000000000000000);
        assert_eq!(m.v_reg[0xF], 0);

        // Wrapping in horizontal direction
        let mut m = Machine::new();
        m.v_reg[0] = 63;
        m.v_reg[1] = 0;
        m.i_reg = m.font[0];
        m.execute_instruction(0xD015);
        assert_eq!(m.display[0], 0xE000000000000001);
        assert_eq!(m.display[1], 0x2000000000000001);
        assert_eq!(m.display[2], 0x2000000000000001);
        assert_eq!(m.display[3], 0x2000000000000001);
        assert_eq!(m.display[4], 0xE000000000000001);
        assert_eq!(m.v_reg[0xF], 0);

        // Test that n is used. (Last row should be unchanged)
        let mut m = Machine::new();
        m.i_reg = m.font[0];
        m.v_reg[0] = 0;
        m.execute_instruction(0xD004);
        assert_eq!(m.display[0], 0xF000000000000000);
        assert_eq!(m.display[1], 0x9000000000000000);
        assert_eq!(m.display[2], 0x9000000000000000);
        assert_eq!(m.display[3], 0x9000000000000000);
        assert_eq!(m.display[4], 0x0000000000000000);
        assert_eq!(m.v_reg[0xF], 0);

        // Test collision
        let mut m = Machine::new();
        m.i_reg = m.font[0];
        m.v_reg[0] = 0;
        m.display[0] = 0x8000000000000000;
        m.execute_instruction(0xD005);
        assert_eq!(m.display[0], 0x7000000000000000);
        assert_eq!(m.display[1], 0x9000000000000000);
        assert_eq!(m.display[2], 0x9000000000000000);
        assert_eq!(m.display[3], 0x9000000000000000);
        assert_eq!(m.display[4], 0xF000000000000000);
        assert_eq!(m.v_reg[0xF], 1);

    }
}