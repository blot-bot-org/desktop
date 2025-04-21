use bbcore::hardware::PhysicalDimensions;
use bbcore::drawing::DrawMethod;
use bbcore::drawing::scribble::{ScribbleMethod, ScribbleParameters};
use bbcore::preview::generate_preview;
use bbcore::instruction::InstructionSet;

#[tauri::command(async)]
fn gen_preview(app: tauri::AppHandle, json_params: &str) -> String {
    let phys_dim = PhysicalDimensions::new(754., (754. - 210.) / 1.98, 192., 210., 297.);

    let params: ScribbleParameters = serde_json::from_str(json_params).unwrap();
    let lines = ScribbleMethod {};

    let ins_bytes = lines.gen_instructions(&phys_dim, &params);
    let instruction_set = InstructionSet::new(ins_bytes).unwrap();
    
    // directory handling
    let cache_dir = tauri::Manager::path(&app).app_cache_dir().expect("Should get cache dir");
    let _ = std::fs::create_dir_all(&cache_dir).map_err(|s| s.to_string());

    let preview_path = cache_dir.join("preview.png");

    generate_preview((0., 0.), &phys_dim, &instruction_set, preview_path.to_str().unwrap());

    preview_path.to_str().unwrap().to_owned()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![gen_preview])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
