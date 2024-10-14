use std::time::SystemTime;

pub fn get_elapsed_time(now: &SystemTime) -> f32 {
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            return elapsed.as_secs_f32();
        }
        Err(_) => {
            // an error occurred!
            return 0.0;
        }
    }
}
