
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

            _ => {},
        }
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

//    #[test]
//    fn test_machine_execute() {
//        let identity = | m: &mut Machine | {};
//
//        let instructions = [
//            (0x0000 as u16, identity, | m: &Machine| -> bool { return true; }), // 0x0nnn instuction is ignored
//            ];
//
//        for (instruction, process, validator) in instructions.iter() {
//            let mut machine = Machine::new();
//            process(&mut machine);
//            machine.execute(*instruction as u16);
//            assert!(validator(&machine));
//
//        }
//    }


}