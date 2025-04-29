use bbcore::drawing::lines::{LinesMethod, LinesParameters};
use bbcore::drawing::cascade::{CascadeMethod, CascadeParameters};
use bbcore::drawing::scribble::{ScribbleMethod, ScribbleParameters};
use bbcore::hardware::PhysicalDimensions;
use bbcore::drawing::DrawMethod;
use bbcore::preview::generate_preview;
use bbcore::instruction::InstructionSet;
use std::fs::File;
use std::io::{Read, Write};
use tokio::sync::Mutex;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf, ReadHalf, WriteHalf};
use std::sync::Arc;
use tauri::State;
use bbcore::client::error::ClientError;
use bbcore::client::state::ClientState;

#[tauri::command(async)]
fn gen_preview(app: tauri::AppHandle, style_id: &str, json_params: &str) -> String {
    let phys_dim = PhysicalDimensions::new(754., (754. - 210.) / 1.98, 192., 210., 297.);
    
    let ins_bytes: Vec<u8> = match style_id {
        "cascade" => {
            let params: CascadeParameters = serde_json::from_str(json_params).unwrap();
            let method = CascadeMethod {};
            method.gen_instructions(&phys_dim, &params)
        },
        "lines" => {
            let params: LinesParameters = serde_json::from_str(json_params).unwrap();
            let method = LinesMethod {};
            method.gen_instructions(&phys_dim, &params)
        },
        "scribble" => {
            let params: ScribbleParameters = serde_json::from_str(json_params).unwrap();
            let method = ScribbleMethod {};
            method.gen_instructions(&phys_dim, &params)
        }
        _ => {
            vec![]
        }
    };

    let instruction_set = InstructionSet::new(ins_bytes).unwrap();

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
    let cache_dir = tauri::Manager::path(&app).app_cache_dir().expect("Should get cache dir");
    let _ = std::fs::create_dir_all(&cache_dir).map_err(|s| s.to_string());
    let ins_file_path = cache_dir.join("instructions.bin");
    let mut ins_file = File::open(ins_file_path).unwrap();
    let mut buffer = Vec::new();
    let _ = ins_file.read_to_end(&mut buffer).unwrap();

    match InstructionSet::new(buffer) {
        Ok(val) => { /* TODO draw(val) */ },
        Err(e) => { println!("{e}"); return Err(e.to_string()); },
    };

    // create new socket, and split it into owned directions
    let client = ClientState::new("192.168.0.16", 8180).await.unwrap();
    let (stream_reader, stream_writer) = client.into_split();
    
    // lock writer, set writer as owned, drop it
    let mut writer_lock = state.writer.lock().await;
    *writer_lock = Some(stream_writer);
    drop(writer_lock);

    // lock reader, set writer as owned, launch listen task
    let mut reader_lock = state.reader.lock().await;
    *reader_lock = Some(stream_reader);
    let reader = reader_lock.as_mut().unwrap();

    ClientState::listen(reader, &state.writer).await;

    Ok("".to_owned())
}

#[tauri::command(async)]
async fn pause_firmware(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String>  {
    let mut writer_lock = state.writer.lock().await;
    let writer = writer_lock.as_mut().unwrap();

    ClientState::pause(writer).await;

    Ok("".to_owned())
}


pub struct AppState {
    pub writer: Arc<Mutex<Option<OwnedWriteHalf>>>,
    pub reader: Arc<Mutex<Option<OwnedReadHalf>>>,
    pub awaiting_pause: Arc<Mutex<bool>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState { writer: Arc::new(Mutex::new(None)), reader: Arc::new(Mutex::new(None)), awaiting_pause: Arc::new(Mutex::new(false)) };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![gen_preview, send_to_firmware, pause_firmware])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

