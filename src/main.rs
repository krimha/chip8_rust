mod chip8;

//use std::thread;
use std::time::Duration;
use std::io::{self,Read};

extern crate sdl2;
use sdl2::{keyboard, pixels::Color};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const SCALE: u32 = 20;
const WIDTH: u32 = 64 * SCALE;
const HEIGHT: u32 = 32 * SCALE;
 


pub fn main() {
    let mut machine = chip8::Machine::new();
    let mut file = std::fs::File::open("IBM_Logo.ch8").unwrap();
    // let mut file = std::fs::File::open("test_opcode.ch8").unwrap();
    //let mut file = std::fs::File::open("BC_test.ch8").unwrap();

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    for (i, val) in buf.iter().enumerate() {
        machine.memory[machine.program_counter as usize + i] = *val;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {

        let instruction:u16 = ((machine.memory[machine.program_counter as usize] as u16) << 8)
                            | machine.memory[machine.program_counter as usize + 1] as u16;

        machine.execute_instruction(instruction);
        machine.program_counter += 2;

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        machine.render_display(&mut canvas, SCALE);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(kcode), ..} => {                        
                    match kcode {
                        Keycode::Num1 => machine.keyboard[0x1] = true,
                        Keycode::Num2 => machine.keyboard[0x2] = true,
                        Keycode::Num3 => machine.keyboard[0x3] = true,
                        Keycode::Num4 => machine.keyboard[0xC] = true,

                        Keycode::Q    => machine.keyboard[0x4] = true,
                        Keycode::W    => machine.keyboard[0x5] = true,
                        Keycode::E    => machine.keyboard[0x6] = true,
                        Keycode::R    => machine.keyboard[0xD] = true,

                        Keycode::A    => machine.keyboard[0x7] = true,
                        Keycode::S    => machine.keyboard[0x8] = true,
                        Keycode::D    => machine.keyboard[0x9] = true,
                        Keycode::F    => machine.keyboard[0xE] = true,

                        Keycode::Z    => machine.keyboard[0xA] = true,
                        Keycode::X    => machine.keyboard[0x0] = true,
                        Keycode::C    => machine.keyboard[0xB] = true,
                        Keycode::V    => machine.keyboard[0xF] = true,
                        _ => {},
                    }
                },
                Event::KeyUp { keycode: Some(kcode), ..} => {                        
                    match kcode {
                        Keycode::Num1 => machine.keyboard[0x1] = false,
                        Keycode::Num2 => machine.keyboard[0x2] = false,
                        Keycode::Num3 => machine.keyboard[0x3] = false,
                        Keycode::Num4 => machine.keyboard[0xC] = false,

                        Keycode::Q    => machine.keyboard[0x4] = false,
                        Keycode::W    => machine.keyboard[0x5] = false,
                        Keycode::E    => machine.keyboard[0x6] = false,
                        Keycode::R    => machine.keyboard[0xD] = false,

                        Keycode::A    => machine.keyboard[0x7] = false,
                        Keycode::S    => machine.keyboard[0x8] = false,
                        Keycode::D    => machine.keyboard[0x9] = false,
                        Keycode::F    => machine.keyboard[0xE] = false,

                        Keycode::Z    => machine.keyboard[0xA] = false,
                        Keycode::X    => machine.keyboard[0x0] = false,
                        Keycode::C    => machine.keyboard[0xB] = false,
                        Keycode::V    => machine.keyboard[0xF] = false,
                        _ => {},
                    }
                }
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60 ));
    }



    return ();





    machine.i_reg = machine.font[0];
    machine.v_reg[1] = 2;
    for _ in 0..4 {
        machine.v_reg[0] = 0;
        for _ in 0..4 {
            machine.v_reg[0] += 12;
            machine.execute_instruction(0xD015);
            machine.i_reg += 5;
        }
        machine.v_reg[1] += 5+2;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        machine.render_display(&mut canvas, SCALE);
        //canvas.fill_rect(Rect::new((0*SCALE) as i32,
        //                           (0*SCALE) as i32, 
        //                           SCALE, 
        //                          SCALE)).unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}