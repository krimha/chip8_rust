
// TODO: Isn't keyboard supposed to be put anywhere in memory?
// TODO: Same "problem" with display


// CHIP-8 Machine state
pub struct Machine {
    memory: [u8; 0x100],  // Main memory (4k bytes)
    v_reg: [u8; 16],      // Vx -registers (0...F)
    i_reg: u16,           // I-register used for memory addresses
    program_counter: u16, // Program counter. Points to current addresss    
    stack_pointer: u8,    // Stack pointer. Points to top of stack

    stack: [u16; 16],

    font_locations: [u16; 16], // Addresses to the font sprites. Same type as i_reg, as they are mem adresses

    delay_timer_register: u8,
    sound_timer_register: u8,

    // TODO: Timers
}


impl Machine {
    // TODO: Derive/Implement Default instead?
    pub fn new() -> Machine {
        Machine {
            memory: [0; 0x100],
            v_reg: [0; 16],
            i_reg: 0,
            program_counter: 0x200,
            stack_pointer: 0,
            stack: [0; 16],
            font_locations:[0x000, 0x005, 0x00a, 0x00f, 0x014, 0x019, 0x01e, 0x023, 0x028, 0x02d, 0x032, 0x037, 0x03c, 0x041, 0x046, 0x04b],
            delay_timer_register: 0,
            sound_timer_register: 0,
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

        for (i, val) in machine.font_locations.iter().enumerate() {
            assert_eq!(*val, (i as u16)*5);
        }
    }
}