use bbcore::drawing::lines::{LinesMethod, LinesParameters};
use bbcore::drawing::cascade::{CascadeMethod, CascadeParameters};
use bbcore::drawing::scribble::{ScribbleMethod, ScribbleParameters};
use bbcore::hardware::PhysicalDimensions;
use bbcore::drawing::DrawMethod;
use bbcore::preview::generate_preview;
use bbcore::instruction::InstructionSet;
use std::fs::File;
use std::io::{Read, Write};
use bbcore::client::state::ClientState;
use tokio::sync::{Mutex, MutexGuard};
use std::sync::Arc;
use tauri::State;

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

    let client = ClientState::new("192.168.0.16", 8180).await.unwrap();

    let mut client_lock = state.client.lock().await;
    *client_lock = Some(client);
    let client = client_lock.as_mut().unwrap();
    
    tokio::spawn(async move {
        ClientState::listen(client).await;
    });

    Ok("".to_owned())
}


#[tauri::command(async)]
async fn pause_firmware(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<String, String>  {
    println!("acquiring mutex...");
    let mut client_lock = state.client.lock().await;
    println!("got the lock...");

    if let Some(client) = client_lock.as_mut() {
        println!("SENDING PAUSE...");
        ClientState::pause(client).await;
    } else {
        println!("COULDNT UNWRAP. NO CLIENT OR SOMMING?");
    }

    Ok("".to_owned())
}

struct AppState {
    pub client: Arc<Mutex<Option<ClientState>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState { client: Arc::new(Mutex::new(None)) };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![gen_preview, send_to_firmware, pause_firmware])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
