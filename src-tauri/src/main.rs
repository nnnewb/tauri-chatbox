// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod session;
use session::Database;
use session::Message;
use session::Session;

fn main() {
    tauri_app_lib::run()
}
