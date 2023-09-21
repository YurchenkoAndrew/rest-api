use chrono::Utc;


pub fn set_current_time() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}