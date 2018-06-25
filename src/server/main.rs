extern crate user32;
extern crate winapi;
extern crate winit;

use std::net::UdpSocket;
use winit::os::windows::WindowExt;
use std::io::Write;

use winit::{Event, WindowEvent, VirtualKeyCode, ElementState};

fn main() {
    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .build(&events_loop).unwrap();

    window.set_title("A fantastic window!");
    window.set_cursor_state(winit::CursorState::Hide).unwrap();

    let stream = UdpSocket::bind("0.0.0.0:13112").expect("Can't connect to port: ");

    let mut previous_key_state: Vec<bool> = Vec::new();
    previous_key_state.resize(255, false);
    //stream.connect("192.168.1.72:13111").expect("Failed to connect");
    stream.connect("127.0.0.1:13111").expect("Failed to connect");

//    let is_in_another_window = std::sync::Arc::new(std::sync::Mutex::new(42));
//    let clone_thing = is_in_another_window.clone();
//    std::thread::spawn(move || other_thread(clone_thing));

    events_loop.run_forever(|event| {
//        println!("{:?}", event);
//
//        {
//            let mut data = is_in_another_window.lock().unwrap();
//            *data += 1;
//        }

        window.set_cursor_position(100, 100).expect("Error setting cursor pos");
//        for i in 1..255u8 {
//            let mut is_current_key_down: bool;
//            unsafe {
//                is_current_key_down = user32::GetAsyncKeyState(i as i32) & (1 << 15) != 0;
//            }
//            if previous_key_state[i as usize] != is_current_key_down {
//                previous_key_state[i as usize] = is_current_key_down;
//                stream.send(&[i, is_current_key_down as u8]).expect("Failed to write");
//            }
//        }
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => winit::ControlFlow::Break,
            Event::WindowEvent { event: WindowEvent::KeyboardInput {device_id, input}, ..} => {
                let key_code = if let Some(keycode) = input.virtual_keycode {
                    keycode as u8 + 8
                } else {
                    200
                };

                if previous_key_state[key_code as usize] != (input.state == ElementState::Pressed) {
                    stream.send(&[key_code, input.state as u8]).expect("Failed to write");
                    previous_key_state[key_code as usize] = (input.state == ElementState::Pressed);
                    println!("{}", key_code);
                }
                winit::ControlFlow::Continue
            },
            _ => winit::ControlFlow::Continue,
        }
    });
}

//fn other_thread(other_window: std::sync::Arc<std::sync::Mutex<i32>>) {
//    loop {
//        std::thread::sleep(std::time::Duration::from_millis(20));
//        let mut cursor_pos = winapi::windef::POINT { x: 0, y: 0 };
//        println!("Thread num: {}", other_window.lock().unwrap());
//        unsafe {
//            //let data = std::ptr::read(window.get_hwnd());
//
//            //std::ptr::write(temp, window.get_hwnd());
//            //user32::SetForegroundWindow(window.get_hwnd() as *mut _);
//            //user32::ShowWindow(window.get_hwnd() as *mut _, 6);
//        }
////        unsafe {
////            if user32::GetCursorPos(&mut cursor_pos as winapi::windef::LPPOINT) != 0 {
////                user32::SetCursorPos(500,500);
////                user32::ShowCursor(0);
////            }
////        }
////
////user32::SetWindowsHookExA()
////        for i in 1..255u8 {
////            let mut is_current_key_down: bool;
////            unsafe {
////                is_current_key_down = user32::GetAsyncKeyState(i as i32) & (1 << 15) != 0;
////            }
////            if previous_key_state[i as usize] != is_current_key_down {
////                previous_key_state[i as usize] = is_current_key_down;
////                stream.send(&[i, is_current_key_down as u8]).expect("Failed to write");
////            }
////        }
//    }
//}
