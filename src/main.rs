#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]

use std::{
    mem,
    sync::{Arc, Mutex},
    process::Command,
};
use crossbeam_channel::{bounded, select, unbounded, Receiver, Sender};
use lazy_static::lazy_static;
use winapi::um::winuser::{
    TrackMouseEvent, TME_LEAVE, TRACKMOUSEEVENT,
};
use crate::hotkey::{spawn_hotkey_thread, HotkeyType};
use crate::window::{Window};
use crate::event::{spawn_foreground_hook};

mod hotkey;
mod window;
mod event;
mod common;
mod config;
mod autostart;

lazy_static! {
    static ref CHANNEL: (Sender<Message>, Receiver<Message>) = unbounded();
    static ref CONFIG: Arc<Mutex<config::Config>> = Arc::new(Mutex::new(config::load_config()));
}

pub enum Message {
    HotkeyPressed(HotkeyType),
    TrackMouse(Window),
    ActiveWindowChange(Window),
    MouseLeft,
    Exit,
}

#[macro_export]
macro_rules! str_to_wide {
    ($str:expr) => {{
        $str.encode_utf16()
            .chain(std::iter::once(0))
            .collect::<Vec<_>>()
    }};
}

fn main() {
    let receiver = &CHANNEL.1.clone();
    let sender = &CHANNEL.0.clone();
    let close_channel = bounded::<()>(3);

    let config = CONFIG.lock().unwrap().clone();

    unsafe {
        autostart::toggle_autostart_registry_key(config.auto_start);
    }

    let mut track_mouse = false;

    spawn_hotkey_thread(&"SHIFT+ALT+RETURN", HotkeyType::Main);
    spawn_hotkey_thread(&"SHIFT+ALT+E", HotkeyType::Second);

    spawn_foreground_hook(close_channel.1.clone());

    loop {
        select! {
            recv(receiver) -> msg => {
                match msg.unwrap() {
                    Message::HotkeyPressed(hotkey_type) => {
                        println!("{}", "HOT");
                        if hotkey_type == HotkeyType::Main {
                            Command::new("wezterm")
                                .spawn()
                                .expect("failed to execute process");
                        } else  if hotkey_type == HotkeyType::Second {
                            println!("{}", "SECOND");
                        } else {
                            println!("{}", "Q");
                            let _ = sender.send(Message::Exit);
                        }
                    }
                    Message::TrackMouse(window) => unsafe {
                        if !track_mouse {
                            let mut event_track: TRACKMOUSEEVENT = mem::zeroed();
                            event_track.cbSize = mem::size_of::<TRACKMOUSEEVENT>() as u32;
                            event_track.dwFlags = TME_LEAVE;
                            event_track.hwndTrack = window.0;

                            TrackMouseEvent(&mut event_track);

                            track_mouse = true;
                        }
                        println!("{:?}", window);
                    }
                    Message::ActiveWindowChange(window) => {
                        println!("{:?}", window);
                    }
                    Message::MouseLeft => {
                        track_mouse = false;
                    }
                    Message::Exit => {
                        break;
                    }
                }
            },
        }
    }
}
