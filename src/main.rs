mod chip8;

use std::{collections::HashMap, hash::Hash};

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
    let mut keymap = HashMap::new();
    keymap.insert(Keycode::Num1, 0x1);
    keymap.insert(Keycode::Num2, 0x2);
    keymap.insert(Keycode::Num3, 0x3);
    keymap.insert(Keycode::Num4, 0xC);
    keymap.insert(Keycode::Q,    0x4);
    keymap.insert(Keycode::W,    0x5);
    keymap.insert(Keycode::E,    0x6);
    keymap.insert(Keycode::R,    0xD);
    keymap.insert(Keycode::A,    0x7);
    keymap.insert(Keycode::S,    0x8);
    keymap.insert(Keycode::D,    0x9);
    keymap.insert(Keycode::F,    0xE);
    keymap.insert(Keycode::Z,    0xA);
    keymap.insert(Keycode::X,    0x0);
    keymap.insert(Keycode::C,    0xB);
    keymap.insert(Keycode::V,    0xF);

    let mut machine = chip8::Machine::new();
    let mut file = std::fs::File::open("breakout.ch8").unwrap();
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
        // Always decrease timers
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60 ));
        if machine.delay_timer_register > 0 {
            machine.delay_timer_register -= 1;
        }

        if machine.sound_timer_register > 0 {
            machine.sound_timer_register -= 1;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(kcode), ..} => {                        
                    match keymap.get(&kcode) {
                        Some(code) => {
                            machine.keyboard[*code as usize] = true;
                            if machine.keypad_waiting {
                                machine.v_reg[machine.waiting_register] = *code;
                                machine.keypad_waiting = false;
                            }
                        }
                        None => {},
                    }
                },
                Event::KeyUp { keycode: Some(kcode), ..} => {                        
                    match keymap.get(&kcode) {
                        Some(code) => machine.keyboard[*code as usize] = false,
                        None => {},
                    }
                }
                _ => {}
            }
        }

        if machine.keypad_waiting {
            println!("WAITING");
            continue;
        }

        let instruction:u16 = ((machine.memory[machine.program_counter as usize] as u16) << 8)
                            | machine.memory[machine.program_counter as usize + 1] as u16;

        machine.execute_instruction(instruction);
        machine.program_counter += 2;

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        machine.render_display(&mut canvas, SCALE);

        println!("{:?}", machine.keyboard);
        canvas.present();
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
    }
}