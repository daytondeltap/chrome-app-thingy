use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager, Size, LogicalSize, WebviewUrl, WebviewWindowBuilder};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
struct Settings {
    width: u32,
    text_scale: u32,
    opacity: u32,
    always_on_top: bool,
    show_label: bool,
}
impl Default for Settings {
    fn default() -> Self { Self { width:330, text_scale:100, opacity:96, always_on_top:true, show_label:true } }
}

#[derive(Serialize)]
struct State { schedule: Option<Value>, settings: Settings, widget_open: bool }
#[derive(Serialize)]
struct ImportResult { ok: bool, canceled: bool, error: Option<String>, classes: usize }

fn app_dir(app: &AppHandle) -> Result<PathBuf,String> {
    let p=app.path().app_data_dir().map_err(|e|e.to_string())?;
    fs::create_dir_all(&p).map_err(|e|e.to_string())?; Ok(p)
}
fn read_json(path: PathBuf) -> Option<Value> { fs::read_to_string(path).ok().and_then(|s|serde_json::from_str(&s).ok()) }
fn get_schedule_value(app:&AppHandle)->Option<Value>{ app_dir(app).ok().and_then(|d|read_json(d.join("schedule.json"))) }
fn get_settings_value(app:&AppHandle)->Settings{
    app_dir(app).ok().and_then(|d|fs::read_to_string(d.join("settings.json")).ok()).and_then(|s|serde_json::from_str(&s).ok()).unwrap_or_default()
}
fn write_json<T:Serialize>(path:PathBuf,v:&T)->Result<(),String>{ let s=serde_json::to_string_pretty(v).map_err(|e|e.to_string())?;fs::write(path,s).map_err(|e|e.to_string()) }
fn validate_schedule(v:&Value)->Result<usize,String>{
    if v.get("format").and_then(Value::as_str)!=Some("SCHEDULER_SCHEDULE"){return Err("Not a SCHEDULER exported schedule file.".into())}
    let entries=v.get("entries").and_then(Value::as_array).ok_or("Schedule file is missing entries.")?;
    for e in entries {
        if e.get("name").and_then(Value::as_str).is_none() || e.get("date").and_then(Value::as_str).is_none() || e.get("start").and_then(Value::as_f64).is_none() || e.get("end").and_then(Value::as_f64).is_none(){return Err("Schedule contains an invalid class entry.".into())}
    }
    Ok(entries.len())
}

#[tauri::command]
fn get_state(app:AppHandle)->State{ State{schedule:get_schedule_value(&app),settings:get_settings_value(&app),widget_open:app.get_webview_window("widget").map(|w|w.is_visible().unwrap_or(false)).unwrap_or(false)} }

#[tauri::command]
fn import_schedule(app:AppHandle)->ImportResult{
    let file=rfd::FileDialog::new().add_filter("SCHEDULER schedule", &["json"]).pick_file();
    let Some(path)=file else{return ImportResult{ok:false,canceled:true,error:None,classes:0}};
    let data=match fs::read_to_string(&path).ok().and_then(|s|serde_json::from_str::<Value>(&s).ok()) {Some(v)=>v,None=>return ImportResult{ok:false,canceled:false,error:Some("Could not read that file.".into()),classes:0}};
    let n=match validate_schedule(&data){Ok(n)=>n,Err(e)=>return ImportResult{ok:false,canceled:false,error:Some(e),classes:0}};
    match app_dir(&app).and_then(|d|write_json(d.join("schedule.json"),&data)){Ok(_)=>ImportResult{ok:true,canceled:false,error:None,classes:n},Err(e)=>ImportResult{ok:false,canceled:false,error:Some(e),classes:0}}
}

#[tauri::command]
fn save_settings(app:AppHandle, mut settings:Settings)->Result<(),String>{
    settings.width=settings.width.clamp(260,520);settings.text_scale=settings.text_scale.clamp(80,140);settings.opacity=settings.opacity.clamp(70,100);
    write_json(app_dir(&app)?.join("settings.json"),&settings)?;
    if let Some(w)=app.get_webview_window("widget") { let _=w.set_size(Size::Logical(LogicalSize::new(settings.width as f64, if settings.show_label {150.0}else{132.0})));let _=w.set_always_on_top(settings.always_on_top); }
    Ok(())
}

#[tauri::command]
fn open_widget(app:AppHandle)->Result<(),String>{
    if let Some(w)=app.get_webview_window("widget"){w.show().map_err(|e|e.to_string())?;return Ok(())}
    let s=get_settings_value(&app);let h=if s.show_label{150.0}else{132.0};
    WebviewWindowBuilder::new(&app,"widget",WebviewUrl::App("widget.html".into()))
        .title("SCHEDULER Widget").inner_size(s.width as f64,h).resizable(false).decorations(false).transparent(true).shadow(false).skip_taskbar(true).always_on_top(s.always_on_top).build().map_err(|e|e.to_string())?;
    Ok(())
}
#[tauri::command]
fn hide_widget(app:AppHandle)->Result<(),String>{if let Some(w)=app.get_webview_window("widget"){w.hide().map_err(|e|e.to_string())?}Ok(())}
#[tauri::command]
fn show_control(app:AppHandle)->Result<(),String>{if let Some(w)=app.get_webview_window("main"){w.show().map_err(|e|e.to_string())?;let _=w.set_focus();}Ok(())}
#[tauri::command]
fn clear_schedule(app:AppHandle)->Result<(),String>{if let Ok(d)=app_dir(&app){let _=fs::remove_file(d.join("schedule.json"));}Ok(())}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(){
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_state,import_schedule,save_settings,open_widget,hide_widget,show_control,clear_schedule])
        .setup(|app|{
            let handle=app.handle().clone();
            if get_schedule_value(&handle).is_some(){let _=open_widget(handle);}
            Ok(())
        })
        .run(tauri::generate_context!()).expect("error while running SCHEDULER");
}
