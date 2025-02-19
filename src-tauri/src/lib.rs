// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod session;
use session::Database;
use session::Message;
use session::Session;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db = Database::new("chatbox.sqlite").expect("unable to create database connection.");
    let app_state = AppState::new(Mutex::new(db));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            create_session,
            get_all_sessions,
            delete_session,
            update_session,
            add_message,
            delete_message,
            get_all_messages,
            get_session,
            clear_messages // 注册新的命令
        ])
        .manage(app_state)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

struct AppState {
    db: Mutex<Database>,
}

impl AppState {
    pub fn new(db: Mutex<Database>) -> Self {
        Self { db }
    }
}

#[tauri::command]
fn create_session(state: tauri::State<AppState>, name: &str) -> Result<Session, String> {
    state
        .db
        .lock()
        .unwrap()
        .add_session(name)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn delete_session(state: tauri::State<AppState>, id: i32) -> Result<(), String> {
    state
        .db
        .lock()
        .unwrap()
        .delete_session(id)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn update_session(state: tauri::State<AppState>, id: i32, name: &str) -> Result<(), String> {
    state
        .db
        .lock()
        .unwrap()
        .update_session(id, name)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn get_all_sessions(state: tauri::State<AppState>) -> Result<Vec<Session>, String> {
    state
        .db
        .lock()
        .unwrap()
        .get_all_sessions()
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn get_session(state: tauri::State<AppState>, id: i32) -> Result<Session, String> {
    state
        .db
        .lock()
        .unwrap()
        .get_session(id)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn get_all_messages(
    state: tauri::State<AppState>,
    session_id: i32,
) -> Result<Vec<Message>, String> {
    state
        .db
        .lock()
        .unwrap()
        .get_all_messages(session_id)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn delete_message(state: tauri::State<AppState>, message_id: i32) -> Result<(), String> {
    state
        .db
        .lock()
        .unwrap()
        .delete_message(message_id)
        .map_err(|err| err.to_string())
}

#[tauri::command]
fn add_message(
    state: tauri::State<AppState>,
    session_id: i32,
    role: &str,
    text: Option<&str>,
    attachment_path: Option<&str>,
) -> Result<Message, String> {
    state
        .db
        .lock()
        .unwrap()
        .add_message(session_id, role, text, attachment_path)
        .map_err(|err| err.to_string())
}

// 新增命令：清空会话消息
#[tauri::command]
fn clear_messages(state: tauri::State<AppState>, session_id: i32) -> Result<(), String> {
    state
        .db
        .lock()
        .unwrap()
        .clear_messages(session_id)
        .map_err(|err| err.to_string())
}