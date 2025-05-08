use bbcore::drawing::lines::{LinesMethod, LinesParameters};
use bbcore::drawing::cascade::{CascadeMethod, CascadeParameters};
use bbcore::drawing::scribble::{ScribbleMethod, ScribbleParameters};
use bbcore::drawing::dunes::{DunesMethod, DunesParameters};
use bbcore::drawing::islands::{IslandsMethod, IslandsParameters};
use bbcore::drawing::bubbles::{BubblesMethod, BubblesParameters};
use bbcore::drawing::waves::{WavesMethod, WavesParameters};
use bbcore::hardware::PhysicalDimensions;
use bbcore::drawing::DrawMethod;
use bbcore::preview::generate_preview;
use bbcore::instruction::InstructionSet;
use std::fs::File;
use std::io::Write;
use tokio::sync::Mutex;
use std::sync::Arc;

pub mod file;
pub mod client;


/// 
/// A Tauri command used to generate a preview of a drawing, save the instructions,
/// and returns the path to the preview image.
///
/// # Parameters:
/// - `app`: Injected dependency from Tauri
/// - `style_id`: The drawing method ID
/// - `json_params`: The drawing method parameters, as JSON
///
/// # Returns:
/// - A path pointing to the preview image
///
#[tauri::command(async)]
fn gen_preview(app: tauri::AppHandle, style_id: &str, json_params: &str) -> String {
    let phys_dim = PhysicalDimensions::new(754., (754. - 210.) / 1.98, 192., 210., 297.);
    
    let ins_bytes: Result<(Vec<u8>, f64, f64), String> = match style_id {
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
        "bubbles" => {
            let params = match serde_json::from_str::<BubblesParameters>(json_params) {
                Ok(val) => val,
                Err(err) => return "error:".to_owned() + err.to_string().as_str(),
            };
            let method = BubblesMethod {};
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
        },
        "islands" => {
            let params = match serde_json::from_str::<IslandsParameters>(json_params) {
                Ok(val) => val,
                Err(err) => return "error:".to_owned() + err.to_string().as_str(),
            };
            let method = IslandsMethod {};
            method.gen_instructions(&phys_dim, &params)
        },
        "waves" => {
            let params = match serde_json::from_str::<WavesParameters>(json_params) {
                Ok(val) => val,
                Err(err) => return "error:".to_owned() + err.to_string().as_str(),
            };
            let method = WavesMethod {};
            method.gen_instructions(&phys_dim, &params)
        },
        _ => {
            Err("error:Unknown draw type".to_owned())
        }
    };

    if let Err(err_str) = ins_bytes {

        return format!("error:{}", err_str).to_owned();

    }

    let instruction_set = match ins_bytes {
        Ok((bytes, ix, iy)) => InstructionSet::new(bytes, ix, iy).unwrap(),
        Err(str) => { panic!("{}", str); }
    };

    // directory handling
    let cache_dir = tauri::Manager::path(&app).app_cache_dir().expect("Should get cache dir");
    let _ = std::fs::create_dir_all(&cache_dir).map_err(|s| s.to_string());

    let ins_file_path = cache_dir.join("instructions.bin");
    let mut ins_file = File::create(ins_file_path).unwrap();
    let _ = ins_file.write_all(instruction_set.get_binary().as_slice());

    let start_file_path = cache_dir.join("start.bin");
    let mut start_file = File::create(start_file_path).unwrap();
    let _ = start_file.write_all(format!("{} {}", instruction_set.get_init().0, instruction_set.get_init().1).as_bytes());

    let preview_path = cache_dir.join("preview.png");

    generate_preview((instruction_set.get_init().0, instruction_set.get_init().1), &phys_dim, &instruction_set, preview_path.to_str().unwrap());

    preview_path.to_str().unwrap().to_owned()
}



/// 
/// Entry point function for the Tauri app
///
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = client::AppState { writer: Arc::new(Mutex::new(None)), reader: Arc::new(Mutex::new(None)), paused_flag: Arc::new(Mutex::new(false)), buf_idx: Arc::new(Mutex::new(0)) };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            gen_preview,
            client::send_to_firmware,
            client::pause_firmware,
            client::move_pen_to_start,
            client::stop_drawing,
            file::save_file,
            file::open_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

