use bbcore::drawing::lines::{LinesMethod, LinesParameters};
use bbcore::drawing::cascade::{CascadeMethod, CascadeParameters};
use bbcore::drawing::scribble::{ScribbleMethod, ScribbleParameters};
use bbcore::drawing::dunes::{DunesMethod, DunesParameters};
use bbcore::hardware::PhysicalDimensions;
use bbcore::drawing::DrawMethod;
use bbcore::preview::generate_preview;
use bbcore::instruction::InstructionSet;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::{DerefMut};
use tokio::sync::Mutex;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use std::sync::Arc;
use tauri::{Emitter, Manager, State};
use bbcore::client::state::ClientState;

#[tauri::command(async)]
fn gen_preview(app: tauri::AppHandle, style_id: &str, json_params: &str) -> String {
    let phys_dim = PhysicalDimensions::new(754., (754. - 210.) / 1.98, 192., 210., 297.);
    
    let ins_bytes: Result<Vec<u8>, String> = match style_id {
        "cascade" => {
            let params = match serde_json::from_str::<CascadeParameters>(json_params) {
                Ok(val) => val,
                Err(err) => return "error:".to_owned() + err.to_string().as_str(),
            };
            let method = CascadeMethod {};
            method.gen_instructions(&phys_dim, &params)
        },
        "lines" => {
            let params = match serde_json::from_str::<LinesParameters>(json_params) {
                Ok(val) => val,
                Err(err) => return "error:".to_owned() + err.to_string().as_str(),
            };
            let method = LinesMethod {};
            method.gen_instructions(&phys_dim, &params)
        },
        "scribble" => {
            let params = match serde_json::from_str::<ScribbleParameters>(json_params) {
                Ok(val) => val,
                Err(err) => return "error:".to_owned() + err.to_string().as_str(),
            };
            let method = ScribbleMethod {};
            method.gen_instructions(&phys_dim, &params)
        },
        "dunes" => {
            let params = match serde_json::from_str::<DunesParameters>(json_params) {
                Ok(val) => val,
                Err(err) => return "error:".to_owned() + err.to_string().as_str(),
            };
            let method = DunesMethod {};
            method.gen_instructions(&phys_dim, &params)
        }
        _ => {
            Err("error:Unknown draw type".to_owned())
        }
    };

    if let Err(err_str) = ins_bytes {

        return format!("error:{}", err_str).to_owned();

    }

    let instruction_set = InstructionSet::new(ins_bytes.unwrap()).unwrap();

    // directory handling
    let cache_dir = tauri::Manager::path(&app).app_cache_dir().expect("Should get cache dir");
    let _ = std::fs::create_dir_all(&cache_dir).map_err(|s| s.to_string());

    let ins_file_path = cache_dir.join("instructions.bin");
    let mut ins_file = File::create(ins_file_path).unwrap();
    let _ = ins_file.write_all(instruction_set.get_binary().as_slice());

    let preview_path = cache_dir.join("preview.png");

    generate_preview((0., 0.), &phys_dim, &instruction_set, preview_path.to_str().unwrap());

    preview_path.to_str().unwrap().to_owned()
}

#[tauri::command(async)]
async fn send_to_firmware(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String> {

    let win = app.get_webview_window("main").unwrap();

    let cache_dir = tauri::Manager::path(&app).app_cache_dir().expect("Should get cache dir");
    let _ = std::fs::create_dir_all(&cache_dir).map_err(|s| s.to_string());
    let ins_file_path = cache_dir.join("instructions.bin");
    let mut ins_file = File::open(ins_file_path).unwrap();
    let mut buffer = Vec::new();
    let _ = ins_file.read_to_end(&mut buffer).unwrap();
    
    let ins_set = match InstructionSet::new(buffer) {
        Ok(val) => { val },
        Err(e) => { println!("{e}"); return Err(e.to_string()); },
    };

    let mut buf_idx_lock = state.buf_idx.lock().await;
    *buf_idx_lock = 0;
    drop(buf_idx_lock);

    // create new socket, and split it into owned directions
    let client = ClientState::new("192.168.0.16", 8180).await.unwrap();
    let (stream_reader, stream_writer) = client.into_split();

    win.emit("firm-prog", r#"{"event":"connection", "message":"Machine accepted connection"}"#).unwrap();
    
    // lock writer, set writer as owned, drop it
    let mut writer_lock = state.writer.lock().await;
    *writer_lock = Some(stream_writer);
    drop(writer_lock);

    // lock reader, set writer as owned, launch listen task
    let mut reader_lock = state.reader.lock().await;
    *reader_lock = Some(stream_reader);
    let reader = reader_lock.as_mut().unwrap();

    ClientState::listen(reader, &state.writer, &state.buf_idx, &ins_set, move |msg| { let _ = win.emit("firm-prog", msg.as_str()); }).await;

    let mut writer_lock = state.writer.lock().await;
    *writer_lock = None;
    *reader_lock = None;
    let mut paused_lock = state.paused_flag.lock().await;
    *paused_lock = false;

    drop(paused_lock);
    drop(writer_lock);
    drop(reader_lock);

    println!("Cleanly exited drawing.");

    Ok("".to_owned())
}

#[tauri::command(async)]
async fn pause_firmware(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String>  {
    let mut writer_lock = state.writer.lock().await;
    let writer = writer_lock.as_mut().unwrap();

    let mut paused_lock = state.paused_flag.lock().await;
    *paused_lock.deref_mut() = !(*paused_lock);

    let win = app.get_webview_window("main").unwrap();

    ClientState::pause(writer, *paused_lock, move |msg| { let _ = win.emit("firm-prog", msg.as_str()); }).await;
    
    // drops occur out of scope

    Ok("".to_owned())
}


pub struct AppState {
    pub writer: Arc<Mutex<Option<OwnedWriteHalf>>>,
    pub reader: Arc<Mutex<Option<OwnedReadHalf>>>,
    pub paused_flag: Arc<Mutex<bool>>,
    pub buf_idx: Arc<Mutex<usize>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState { writer: Arc::new(Mutex::new(None)), reader: Arc::new(Mutex::new(None)), paused_flag: Arc::new(Mutex::new(false)), buf_idx: Arc::new(Mutex::new(0)) };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![gen_preview, send_to_firmware, pause_firmware])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

