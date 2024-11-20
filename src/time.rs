use std::time::SystemTime;

pub fn get_elapsed_time(now: &SystemTime) -> f32 {
    match now.elapsed() {
        Ok(elapsed) => elapsed.as_secs_f32(),
        Err(_) => 0.0,
    }
}
