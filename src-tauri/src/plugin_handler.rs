use bbcore::plugin;


/// 
/// Gets the parameters of a plugin, give the plugin file path.
///
/// # Parameters:
/// - `app`: Injected dependency from Tauri
/// - `path`: The path of the plugin in the filesystem
///
/// # Returns:
/// - The plugins parameters as stringified JSON 
/// - A string explaning why the function failed
///
#[tauri::command(async)]
pub async fn get_parameters(_app: tauri::AppHandle, path: &str) -> Result<String, String> {
    match plugin::get_parameter_string(path) {
        Ok(val) => Ok(val),
        Err(err) => Err(format!("Couldn't load plugin parameters: {}", err.to_string()))
    }
}
