use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};

use serde::{Serialize, Deserialize};
use bbcore::drawing::DrawParameters;
use bbcore::drawing::lines::LinesParameters;
use bbcore::drawing::cascade::CascadeParameters;
use bbcore::drawing::scribble::ScribbleParameters;
use bbcore::drawing::dunes::DunesParameters;
use bbcore::drawing::islands::IslandsParameters;
use bbcore::drawing::bubbles::BubblesParameters;
use bbcore::drawing::waves::WavesParameters;

#[derive(Serialize, Deserialize)]
#[serde(bound = "T: Serialize + for<'de2> Deserialize<'de2>")]
struct FsDrawing<T : DrawParameters> {
    drawing_id: String,
    drawing_parameters: T 
}

#[derive(Deserialize)]
struct PreDrawingId {
    drawing_id: String,
}

#[tauri::command(async)]
pub async fn save_file(path: &str, drawing_id: &str, json_params: &str) -> Result<(), String> {

    println!("OBIOUVLSY HERE!");
    let file_handle = File::create(path).unwrap();
    println!("GOT HERE!");

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
            _ => { return Err("No such drawing ID".to_owned()); }
    } {
        Ok(()) => {},
        Err(err) => { return Err(err.to_string().to_owned()); }
    }

    println!("image has been saved");
        
    Ok(())
}

#[tauri::command(async)]
pub async fn open_file(path: &str) -> Result<(String, String), String> {

    let mut file_handle = File::open(path).unwrap();
    let drawing_id: PreDrawingId = match serde_json::from_reader(BufReader::new(&file_handle)) {
        Ok(val) => val,
        Err(err) => { return Err(err.to_string()) }
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
            _ => { Err("Invalid drawing type".to_owned()) }
    } {
        Ok(str) => { return Ok((drawing_id.drawing_id, str)) },
        Err(err) => { return Err(format!("Corrupt save file: {}", err).to_owned()); }
    }
}
