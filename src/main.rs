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
use crate::tray::spawn_sys_tray;

mod hotkey;
mod event;
mod common;
mod config;
mod tray;
mod autostart;
mod window;

lazy_static! {
    static ref CHANNEL: (Sender<Message>, Receiver<Message>) = unbounded();
    static ref CONFIG: Arc<Mutex<config::Config>> = Arc::new(Mutex::new(config::load_config()));
}

pub enum Message {
    HotkeyPressed(HotkeyType, String),
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
    let mut track_mouse = false;
    let receiver = &CHANNEL.1.clone();
    let sender = &CHANNEL.0.clone();
    let close_channel = bounded::<()>(3);

    let config = CONFIG.lock().unwrap().clone();

    unsafe {
        autostart::toggle_autostart_registry_key(config.auto_start);
    }

    for (pos, e) in config.hotkeys.iter().enumerate() {
        let command = config.commands[pos].clone();
        println!("registering hotkey {} {}", e, command);
        spawn_hotkey_thread(e.to_string(), HotkeyType::Main, command);
    }

    spawn_foreground_hook(close_channel.1.clone());

    unsafe {
        spawn_sys_tray();
    }

    println!("{}", "tsttr gestartet!");

    loop {
        select! {
            recv(receiver) -> msg => {
                match msg.unwrap() {
                    Message::HotkeyPressed(hotkey_type, cmd) => {
                        println!("{}", cmd);

                        // split cmd
                        let mut args: Vec<String> = cmd
                            .split(' ')
                            .map(|s| s.trim().to_string())
                            .collect();

                        // reverse so we can pop
                        args.reverse();
                        // get real command from vec
                        let cmd = args.pop().unwrap();
                        // reverse back
                        args.reverse();

                        // execude cmd
                        if hotkey_type == HotkeyType::Main {
                            Command::new(cmd.clone())
                                .args(&args)
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
