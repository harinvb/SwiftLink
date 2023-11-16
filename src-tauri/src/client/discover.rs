
static mut i:i32 = 0;

#[tauri::command]
pub fn get_discovered_clients() -> Result<Vec<String>,String> {
    Ok(vec!["a".to_string(),"b".to_string(),"c".to_string()])
}