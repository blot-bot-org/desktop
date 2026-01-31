use std::io::{BufRead, BufReader, Read};
use std::ops::DerefMut;
use bbcore::client;
use tokio::sync::Mutex;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use std::sync::Arc;
use tauri::{Emitter, Manager, State};
use std::fs::File;
use std::io::Write;

use bbcore::client::state::ClientState;
use bbcore::instruction::InstructionSet;
use bbcore::hardware::PhysicalDimensions;

use crate::file::{get_app_config_struct, AppConfig};


/// 
/// Loads the cached instructions and sends them to the firmware for execution.
/// It emits updates to the window through the `firm-prog` channel.
///
/// # Parameters:
/// - `app`: Injected dependency from Tauri
/// - `state`: A Tauri-injected global state object
///
/// # Returns:
/// - Void if the function succeeded
/// - An error explaining why the function could not succeed
///
#[tauri::command(async)]
pub async fn send_to_firmware(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<(), String> {

    let win = app.get_webview_window("main").unwrap();

    let cache_dir = tauri::Manager::path(&app).app_cache_dir().expect("Should get cache dir");
    let _ = std::fs::create_dir_all(&cache_dir).map_err(|s| s.to_string());

    // getting instruction bytes
    let ins_file_path = cache_dir.join("instructions.bin");
    let mut ins_file = File::open(ins_file_path).unwrap();
    let mut buffer = Vec::new();
    let _ = ins_file.read_to_end(&mut buffer).unwrap();

    let ins_set = match InstructionSet::new(buffer, 0., 0.) { 
        Ok(val) => { val },
        Err(e) => { println!("{e}"); return Err(e.to_string()); },
    };

    let app_config = match get_app_config_struct(&app) {
        Ok(val) => val,
        Err(_) => { return Err("Print settings have not been configured.".to_owned()); }
    };

    let mut buf_idx_lock = state.buf_idx.lock().await;
    *buf_idx_lock = 0;
    drop(buf_idx_lock);


    win.emit("firm-prog", format!(r#"{{"event":"populate_network", "address":"{}"}}"#, format!("{}:{}", app_config.machine_addr, app_config.machine_port))).unwrap();
    win.emit("firm-prog", format!(r#"{{"event":"populate_draw", "totalBytes":"{}"}}"#, ins_set.get_binary().len())).unwrap();

    // create new socket, and split it into owned directions
    let (client, machine_config) = ClientState::new(&app_config.machine_addr, app_config.machine_port).await.unwrap();
    let (stream_reader, stream_writer) = client.into_split();

    win.emit("firm-prog", r#"{"event":"connection", "message":"Machine accepted connection"}"#).unwrap();
    win.emit("firm-prog", format!(r#"{{"event":"populate_machine", "insBytes":"{}", "stepSpeed":"{}", "pulseWidth":"{}", "protocol":"{}"}}"#, machine_config.instruction_buffer_size, machine_config.max_motor_speed, machine_config.min_pulse_width, machine_config.protocol_version)).unwrap();
    
    // lock writer, set writer as owned, drop it
    let mut writer_lock = state.writer.lock().await;
    *writer_lock = Some(stream_writer);
    drop(writer_lock);

    // lock reader, set writer as owned, launch listen task
    let mut reader_lock = state.reader.lock().await;
    *reader_lock = Some(stream_reader);
    let reader = reader_lock.as_mut().unwrap();

    ClientState::listen(reader, &state.writer, &state.buf_idx, &ins_set, &machine_config, move |msg| { let _ = win.emit("firm-prog", msg.as_str()); }).await;

    let mut writer_lock = state.writer.lock().await;
    *writer_lock = None;
    *reader_lock = None;
    let mut paused_lock = state.paused_flag.lock().await;
    *paused_lock = false;
    let mut buf_idx_lock = state.buf_idx.lock().await;
    *buf_idx_lock = 0;

    drop(paused_lock);
    drop(writer_lock);
    drop(reader_lock);
    drop(buf_idx_lock);

    #[cfg(debug_assertions)]
    println!("Cleanly exited drawing.");

    Ok(())
}

/// 
/// Sends a pause command to the firmware.
/// It emits updates to the window through the `firm-prog` channel.
///
/// # Parameters:
/// - `app`: Injected dependency from Tauri
/// - `state`: A Tauri-injected global state object
///
/// # Returns:
/// - Void if the function succeeded
/// - An error explaining why the function could not succeed
///
#[tauri::command(async)]
pub async fn pause_firmware(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<(), String>  {

    let mut writer_lock = state.writer.lock().await;
    let writer = writer_lock.as_mut().unwrap();

    let mut paused_lock = state.paused_flag.lock().await;
    *paused_lock.deref_mut() = !(*paused_lock);

    let win = app.get_webview_window("main").unwrap();

    ClientState::pause(writer, *paused_lock, move |msg| { let _ = win.emit("firm-prog", msg.as_str()); }).await;
    
    // drops occur out of scope

    Ok(())
}

/// 
/// Moves the pen to the starting position of the drawing.
///
/// # Parameters:
/// - `app`: Injected dependency from Tauri
///
/// # Returns:
/// - Void if the function succeeded
/// - An error explaining why the function could not succeed
///
#[tauri::command(async)]
pub async fn move_pen_to_start(app: tauri::AppHandle) -> Result<(), String>  {

    let cache_dir = tauri::Manager::path(&app).app_cache_dir().expect("Should get cache dir");
    
    // getting start position
    let start_file_path = cache_dir.join("start.bin");
    let start_file = File::open(start_file_path).unwrap();
    let mut start_contents = String::new();
    BufReader::new(start_file).read_to_string(&mut start_contents).unwrap();
    let start_pos: Vec<f64> = start_contents.split_whitespace().filter_map(|s| s.parse::<f64>().ok()).collect();

    let app_config = match get_app_config_struct(&app) {
        Ok(val) => val,
        Err(_) => { return Err("Print settings have not been configured.".to_owned()); }
    };
    let phys_dim = PhysicalDimensions::new(app_config.phys_motor_interspace, app_config.phys_page_left_offset, app_config.phys_page_top_offset, app_config.phys_page_width, app_config.phys_page_height);

    if let Err(str) = bbcore::client::move_to_start(&app_config.machine_addr, app_config.machine_port, &phys_dim, start_pos[0], start_pos[1]) {
        return Err(str.to_string().to_owned());
    }

    Ok(())
}

/// 
/// Sends a stop command to the firmware.
/// It emits updates to the window through the `firm-prog` channel.
///
/// # Parameters:
/// - `app`: Injected dependency from Tauri
/// - `state`: A Tauri-injected global state object
///
/// # Returns:
/// - Void if the function succeeded
/// - An error explaining why the function could not succeed
///
#[tauri::command(async)]
pub async fn stop_drawing(app: tauri::AppHandle, state: State<'_, AppState>) -> Result<(), String>  {
    
    let win = app.get_webview_window("main").unwrap();

    let mut writer_lock = state.writer.lock().await;
    let writer = writer_lock.as_mut().unwrap();
    
    ClientState::stop(writer, move |msg| { let _ = win.emit("firm-prog", msg.as_str()); }).await;

    // will finish ClientState::listen in other thread

    Ok(())

}


/// 
/// Estimates the drawing time, used for the 'Print' button.
///
/// # Parameters:
/// - `app`: Injected dependency from Tauri
///
/// # Returns:
/// - A u64 representing the estimated time taken to draw a drawing at 500s/s, or 0
///
#[tauri::command(async)]
pub async fn get_image_stats(app: tauri::AppHandle) -> (u64, usize) {

    let cache_dir = tauri::Manager::path(&app).app_cache_dir().expect("Should get cache dir");
    let _ = std::fs::create_dir_all(&cache_dir).map_err(|s| s.to_string());

    // getting instruction bytes
    let ins_file_path = cache_dir.join("instructions.bin");
    let mut ins_file = File::open(ins_file_path).unwrap();
    let mut buffer = Vec::new();
    let _ = ins_file.read_to_end(&mut buffer).unwrap();

    let ins_set = match InstructionSet::new(buffer, 0., 0.) { 
        Ok(val) => { val },
        Err(e) => { println!("{e}"); return (0, 0); },
    };

    let dur = client::calculate_draw_time(&ins_set.get_binary(), 500, 0);

    (dur.as_secs(), ins_set.get_binary().len())
}


/// 
/// A thread-safe global state containing values of the drawing state.
///
/// # Fields:
/// - `writer`: Mutex-guarded write half of a TcpStream
/// - `reader`: Mutex-guarded read half of a TcpStream
/// - `paused_flag`: Mutex-guarded flag to represent whether the machine is paused or not
/// - `buf_idx`: Mutex-guarded usize representing the current buffer bound index
///
pub struct AppState {
    pub writer: Arc<Mutex<Option<OwnedWriteHalf>>>,
    pub reader: Arc<Mutex<Option<OwnedReadHalf>>>,
    pub paused_flag: Arc<Mutex<bool>>,
    pub buf_idx: Arc<Mutex<usize>>,
}



/// 
/// A function used to save the machine config to the disk.
///
/// # Parameters:
/// - `app`: Injected dependency from Tauri
/// - `_`: The app state
/// - `addr`: The user-configured IP address of the machine
/// - `port`: The user-configured port of the machine to save
///
/// # Returns:
/// - Ok() if the function succeeded
/// - A string explaning why the function failed
///
#[tauri::command(async)]
pub async fn save_machine_config(app: tauri::AppHandle, _: State<'_, AppState>, addr: &str, port: usize) -> Result<(), String>  {

    // directory handling
    let cache_dir = tauri::Manager::path(&app).app_cache_dir().expect("Should get cache dir");
    let _ = std::fs::create_dir_all(&cache_dir).map_err(|s| s.to_string());

    let ins_file_path = cache_dir.join("machine_conf");
    let mut ins_file = File::create(ins_file_path).unwrap();
    let _ = ins_file.write_all(format!("{}:{}", addr, port).as_bytes());
    
    Ok(())

}


/// 
/// A function used to load the machine config from the disk.
///
/// # Parameters:
/// - `app`: Injected dependency from Tauri
/// - `_`: The app state
///
/// # Returns:
/// - The user-configured IP and port of the machine in the format `IP:PORT`
/// - Err() if the function failed
///
#[tauri::command(async)]
pub async fn get_machine_config(app: tauri::AppHandle) -> Result<String, ()>  {

    // directory handling
    let cache_dir = tauri::Manager::path(&app).app_cache_dir().expect("Should get cache dir");
    let _ = std::fs::create_dir_all(&cache_dir).map_err(|s| s.to_string());

    let ins_file_path = cache_dir.join("machine_conf");

    match ins_file_path.try_exists() {
        Ok(exists) => if !exists { return Ok("null".to_owned()); },
        Err(_) => { return Ok("null".to_owned()); },
    }

    let ins_file = File::open(ins_file_path).unwrap();
    let mut contents = String::new();

    BufReader::new(ins_file).read_to_string(&mut contents).unwrap();
    if contents.is_empty() {
        return Ok("null".to_owned());
    }
    contents = match contents.strip_suffix("\n") {
        Some(c) => c.to_string(),
        None => contents
    }; // optionally strip newline if it exists

    let parts: Vec<&str> = contents.split(":").collect();

    if parts.len() != 2 {
        return Ok("null".to_owned());
    }
    
    Ok(format!("{}:{}", parts.get(0).unwrap(), parts.get(1).unwrap()).to_owned())
}


/// 
/// A function used to load the machine config from the disk.
///
/// # Parameters:
/// - `app`: Injected dependency from Tauri
/// - `_`: The app state
///
/// # Returns:
/// - The user-configured IP and port of the machine in the format `IP:PORT`
/// - Err() if the function failed
///
pub async fn get_machine_config_noncmd(app: &tauri::AppHandle) -> Result<String, ()>  {

    // directory handling
    let cache_dir = tauri::Manager::path(app).app_cache_dir().expect("Should get cache dir");
    let _ = std::fs::create_dir_all(&cache_dir).map_err(|s| s.to_string());

    let ins_file_path = cache_dir.join("machine_conf");

    match ins_file_path.try_exists() {
        Ok(exists) => if !exists { return Ok("null".to_owned()); },
        Err(_) => { return Ok("null".to_owned()); },
    }

    let ins_file = File::open(ins_file_path).unwrap();
    let mut contents = String::new();

    BufReader::new(ins_file).read_to_string(&mut contents).unwrap();
    if contents.is_empty() {
        return Ok("null".to_owned());
    }
    contents = match contents.strip_suffix("\n") {
        Some(c) => c.to_string(),
        None => contents
    }; // optionally strip newline if it exists

    let parts: Vec<&str> = contents.split(":").collect();

    if parts.len() != 2 {
        return Ok("null".to_owned());
    }
    
    Ok(format!("{}:{}", parts.get(0).unwrap(), parts.get(1).unwrap()).to_owned())
}
