// extern crate sdl2;
//
// use std::process;
// use sdl2::rect::{Rect};
// use sdl2::event::{Event};
// use sdl2::keyboard::Keycode;
//
// fn main() {
//     let ctx = sdl2::init().unwrap();
//     let video_ctx = ctx.video().unwrap();
//
//     let width = 640;
//     let height = 480;
//
//     let window  = match video_ctx
//         .window("rust_to_js", width, height)
//         .position_centered()
//         .opengl()
//         .build() {
//             Ok(window) => window,
//             Err(err)   => panic!("failed to create window: {}", err)
//         };
//
//     let mut renderer = match window
//         .renderer()
//         .build() {
//             Ok(renderer) => renderer,
//             Err(err) => panic!("failed to create renderer: {}", err)
//         };
//
//     let mut rect = Rect::new(10, 10, 10, 10);
//
//     let black = sdl2::pixels::Color::RGB(0, 0, 0);
//     let white = sdl2::pixels::Color::RGB(255, 255, 255);
//
//     let mut events = ctx.event_pump().unwrap();
//
//     let mut main_loop = || {
//         for event in events.poll_iter() {
//             match event {
//                 Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
//                     process::exit(1);
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
//                     rect.x -= 10;
//                     if rect.x < 0 {
//                         rect.x = (width - 10) as i32;
//                     }
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
//                     rect.x += 10;
//                     if rect.x > width as i32 {
//                         rect.x = 10;
//                     }
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
//                     rect.y -= 10;
//                     if rect.y < 0 {
//                         rect.y = (height - 10) as i32;
//                     }
//                 },
//                 Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
//                     rect.y += 10;
//                     if rect.y > height as i32 {
//                         rect.y = 10;
//                     }
//                 },
//                 _ => {}
//             }
//         }
//
//         let _ = renderer.set_draw_color(black);
//         let _ = renderer.clear();
//         let _ = renderer.set_draw_color(white);
//         let _ = renderer.fill_rect(rect);
//         let _ = renderer.present();
//     };
//
//     loop { main_loop(); }
// }
// Emscripten version:
extern crate sdl2;

use std::process;
use sdl2::rect::{Rect};
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

#[cfg(target_os = "emscripten")]
pub mod emscripten;

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let width = 640;
    let height = 480;

    let window  = match video_ctx
        .window("rust_to_js", width, height)
        .position_centered()
        .opengl()
        .build() {
            Ok(window) => window,
            Err(err)   => panic!("failed to create window: {}", err)
        };

    let mut renderer = match window
        .renderer()
        .build() {
            Ok(renderer) => renderer,
            Err(err) => panic!("failed to create renderer: {}", err)
        };

    let mut rect = Rect::new(10, 10, 10, 10);

    let black = sdl2::pixels::Color::RGB(0, 0, 0);
    let white = sdl2::pixels::Color::RGB(255, 255, 255);

    let mut events = ctx.event_pump().unwrap();

    let mut main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    rect.x -= 10;
                    if rect.x < 0 {
                        rect.x = (width - 10) as i32;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    rect.x += 10;
                    if rect.x > width as i32 {
                        rect.x = 10;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    rect.y -= 10;
                    if rect.y < 0 {
                        rect.y = (height - 10) as i32;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    rect.y += 10;
                    if rect.y > height as i32 {
                        rect.y = 10;
                    }
                },
                _ => {}
            }
        }

        let _ = renderer.set_draw_color(black);
        let _ = renderer.clear();
        let _ = renderer.set_draw_color(white);
        let _ = renderer.fill_rect(rect);
        let _ = renderer.present();
    };

    #[cfg(target_os = "emscripten")]
    use emscripten::{emscripten};

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop_callback(main_loop);

    #[cfg(not(target_os = "emscripten"))]
    loop { main_loop(); }
}
