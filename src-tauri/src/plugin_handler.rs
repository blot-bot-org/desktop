use bbcore::plugin;



#[tauri::command(async)]
pub async fn get_parameters(_app: tauri::AppHandle, path: &str) -> Result<String, String> {
    match plugin::get_parameter_string(path) {
        Ok(val) => Ok(val),
        Err(err) => Err(format!("Couldn't load plugin parameters: {}", err.to_string()))
    }
}
