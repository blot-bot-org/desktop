use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};

use serde::{Serialize, Deserialize};
use bbcore::drawing::DrawParameters;
use bbcore::drawing::lines::LinesParameters;
use bbcore::drawing::cascade::CascadeParameters;
use bbcore::drawing::scribble::ScribbleParameters;
use bbcore::drawing::dunes::DunesParameters;
use bbcore::drawing::islands::IslandsParameters;
use bbcore::drawing::bubbles::BubblesParameters;
use bbcore::drawing::waves::WavesParameters;
use bbcore::drawing::entropy::EntropyParameters;

/// 
/// Used to serialize / deserialize a save file, including the drawing method ID.
///
/// # Fields:
/// - `drawing_id`: The drawing method ID
/// - `drawing_parameters`: The drawing parameters
///
#[derive(Serialize, Deserialize)]
#[serde(bound = "T: Serialize + for<'de2> Deserialize<'de2>")]
struct FsDrawing<T : DrawParameters> {
    drawing_id: String,
    drawing_parameters: T 
}

/// 
/// Used to deserialize just the drawing method ID
///
/// # Fields:
/// - `drawing_id`: The drawing method ID
///
#[derive(Deserialize)]
struct PreDrawingId {
    drawing_id: String,
}

/// 
/// A Tauri command to save a drawing method and parameters to a file.
/// It serializes a `FsDrawing` into a string which is saved to a file.
///
/// # Parameters:
/// - `path`: The path to save the file to
/// - `drawing_id`: The drawing method ID
/// - `json_params`: The serialized drawing parameters
///
/// # Returns:
/// - Void if the function succeeded
/// - An error, as a string, explaining why the function did not succeed
///
#[tauri::command(async)]
pub async fn save_file(path: &str, drawing_id: &str, json_params: &str) -> Result<(), String> {

    let file_handle = File::create(path).unwrap();

    match match drawing_id {
            "cascade" => {
                let params = serde_json::from_str::<CascadeParameters>(json_params).unwrap();
                serde_json::to_writer(file_handle, &FsDrawing { drawing_id: drawing_id.to_owned(), drawing_parameters: params } )
            },
            "lines" => {
                let params = serde_json::from_str::<LinesParameters>(json_params).unwrap();
                serde_json::to_writer(file_handle, &FsDrawing { drawing_id: drawing_id.to_owned(), drawing_parameters: params } )
            },
            "bubbles" => {
                let params = serde_json::from_str::<BubblesParameters>(json_params).unwrap();
                serde_json::to_writer(file_handle, &FsDrawing { drawing_id: drawing_id.to_owned(), drawing_parameters: params } )
            },
            "scribble" => {
                let params = serde_json::from_str::<ScribbleParameters>(json_params).unwrap();
                serde_json::to_writer(file_handle, &FsDrawing { drawing_id: drawing_id.to_owned(), drawing_parameters: params } )
            },
            "dunes" => {
                let params = serde_json::from_str::<DunesParameters>(json_params).unwrap();
                serde_json::to_writer(file_handle, &FsDrawing { drawing_id: drawing_id.to_owned(), drawing_parameters: params } )
            },
            "islands" => {
                let params = serde_json::from_str::<IslandsParameters>(json_params).unwrap();
                serde_json::to_writer(file_handle, &FsDrawing { drawing_id: drawing_id.to_owned(), drawing_parameters: params } )
            },
            "waves" => {
                let params = serde_json::from_str::<WavesParameters>(json_params).unwrap();
                serde_json::to_writer(file_handle, &FsDrawing { drawing_id: drawing_id.to_owned(), drawing_parameters: params } )
            },
            "entropy" => {
                let params = serde_json::from_str::<EntropyParameters>(json_params).unwrap();
                serde_json::to_writer(file_handle, &FsDrawing { drawing_id: drawing_id.to_owned(), drawing_parameters: params } )
            },
            _ => { return Err("No such drawing ID".to_owned()); }
    } {
        Ok(()) => {},
        Err(err) => { return Err(format!("Error saving file: {}", err).to_owned()); }
    }

    Ok(())
}

/// 
/// A Tauri command to load a drawing method into a string.
/// It first deserializes just the drawing_id, to determine which object
/// to deserialize into.
///
/// # Parameters:
/// - `path`: The path to save the file to
///
/// # Returns:
/// - (drawing_id, drawing_parameters) both as strings
/// - An error, as a string, explaining why the function did not succeed
///
#[tauri::command(async)]
pub async fn open_file(path: &str) -> Result<(String, String), String> {

    let mut file_handle = File::open(path).unwrap();
    let drawing_id: PreDrawingId = match serde_json::from_reader(BufReader::new(&file_handle)) {
        Ok(val) => val,
        Err(err) => { return Err(format!("Corrupt save file: {}", err).to_owned()); }
    };

    file_handle.seek(SeekFrom::Start(0)).unwrap();
    let buf_read = BufReader::new(&file_handle);
    match match drawing_id.drawing_id.as_str() {
            "cascade" => {
                match serde_json::from_reader::<_, FsDrawing<CascadeParameters>>(buf_read) {
                    Ok(val) => Ok(serde_json::to_string(&val.drawing_parameters).unwrap()),
                    Err(err) => Err(err.to_string())
                }
            },
            "lines" => {
                match serde_json::from_reader::<_, FsDrawing<LinesParameters>>(buf_read) {
                    Ok(val) => Ok(serde_json::to_string(&val.drawing_parameters).unwrap()),
                    Err(err) => Err(err.to_string())
                }
            },
            "bubbles" => {
                match serde_json::from_reader::<_, FsDrawing<BubblesParameters>>(buf_read) {
                    Ok(val) => Ok(serde_json::to_string(&val.drawing_parameters).unwrap()),
                    Err(err) => Err(err.to_string())
                }
            },
            "scribble" => {
                match serde_json::from_reader::<_, FsDrawing<ScribbleParameters>>(buf_read) {
                    Ok(val) => Ok(serde_json::to_string(&val.drawing_parameters).unwrap()),
                    Err(err) => Err(err.to_string())
                }
            },
            "dunes" => {
                match serde_json::from_reader::<_, FsDrawing<DunesParameters>>(buf_read) {
                    Ok(val) => Ok(serde_json::to_string(&val.drawing_parameters).unwrap()),
                    Err(err) => Err(err.to_string())
                }
            },
            "islands" => {
                match serde_json::from_reader::<_, FsDrawing<IslandsParameters>>(buf_read) {
                    Ok(val) => Ok(serde_json::to_string(&val.drawing_parameters).unwrap()),
                    Err(err) => Err(err.to_string())
                }
            },
            "waves" => {
                match serde_json::from_reader::<_, FsDrawing<WavesParameters>>(buf_read) {
                    Ok(val) => Ok(serde_json::to_string(&val.drawing_parameters).unwrap()),
                    Err(err) => Err(err.to_string())
                }
            },
            "entropy" => {
                match serde_json::from_reader::<_, FsDrawing<EntropyParameters>>(buf_read) {
                    Ok(val) => Ok(serde_json::to_string(&val.drawing_parameters).unwrap()),
                    Err(err) => Err(err.to_string())
                }
            },
            _ => { Err("Invalid drawing type".to_owned()) }
    } {
        Ok(str) => { return Ok((drawing_id.drawing_id, str)) },
        Err(err) => { return Err(format!("Corrupt save file: {}", err).to_owned()); }
    }
}



/// 
/// A serializable struct for storing the app configuration.
///
/// # Fields:
/// - `machine_addr`: The address of the drawing machine server
/// - `machine_port`: The address of the drawing machine server
/// - `phys_motor_interspace`: The horizontal distance between the motors
/// - `phys_page_left_offset`: The horizontal distance between the left motor shaft and the top left of the page
/// - `phys_page_top_offset`: The vertical distance between the left motor shaft and the top left of the page
/// - `phys_page_width`: The width of the page
/// - `phys_page_height`: The height of the page
///
#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub machine_addr: String,
    pub machine_port: u16,

    pub phys_motor_interspace: f64,
    pub phys_page_left_offset: f64,
    pub phys_page_top_offset: f64,
    pub phys_page_width: f64,
    pub phys_page_height: f64,
}

#[tauri::command(async)]
pub fn get_app_config(app: tauri::AppHandle) -> Result<String, ()> {

    let cache_dir = tauri::Manager::path(&app).app_cache_dir().expect("Should get cache dir");
    let app_config_path = cache_dir.join("app_config.json");

    match app_config_path.try_exists() {
        Ok(exists) => if !exists { return Err(()) },
        Err(_) => { return Err(()) }
    };

    let file_handle = match File::open(app_config_path) {
        Ok(handle) => handle,
        Err(_) => { return Err(()) }
    };

    let mut contents = String::new();
    BufReader::new(file_handle).read_to_string(&mut contents).unwrap();

    match serde_json::from_str::<AppConfig>(contents.as_str()) {
        Ok(_) => { return Ok(contents) },
        Err(_) => { return Err(()) }
    };

}

#[tauri::command(async)]
pub fn save_app_config(app: tauri::AppHandle, stringified_config: &str) -> Result<(), ()> {

    let cache_dir = tauri::Manager::path(&app).app_cache_dir().expect("Should get cache dir");
    let app_config_path = cache_dir.join("app_config.json");

    
    let mut file_handle = match File::create(app_config_path) {
        Ok(handle) => handle,
        Err(_) => { return Err(()) }
    };

    // check its in the right format (can be serialized)
    match serde_json::from_str::<AppConfig>(stringified_config) {
        Ok(_) => { },
        Err(_) => { return Err(()) }
    };
    
    match file_handle.write_all(stringified_config.as_bytes()) {
        Ok(_) => { return Ok(()); },
        Err(_) => { return Err(()); }
    };
}

pub fn get_app_config_struct(app: &tauri::AppHandle) -> Result<AppConfig, ()> {
    
    let cache_dir = tauri::Manager::path(app).app_cache_dir().expect("Should get cache dir");
    let app_config_path = cache_dir.join("app_config.json");

    match app_config_path.try_exists() {
        Ok(exists) => if !exists { return Err(()) },
        Err(_) => { return Err(()) }
    };

    let file_handle = match File::open(app_config_path) {
        Ok(handle) => handle,
        Err(_) => { return Err(()) }
    };

    let mut contents = String::new();
    BufReader::new(file_handle).read_to_string(&mut contents).unwrap();

    match serde_json::from_str::<AppConfig>(contents.as_str()) {
        Ok(val) => { return Ok(val) },
        Err(_) => { return Err(()) }
    };

}
