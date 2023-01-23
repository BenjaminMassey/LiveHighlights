use inputbot::KeybdKey::*;
use std::time::Instant;

fn main() {

    let initial = Instant::now();

    F8Key.bind(move || {
        if LShiftKey.is_pressed() && LControlKey.is_pressed() {
            let secs_in_min = 60 as f64;

            let end: f64 = initial.elapsed().as_secs() as f64;
            let start: f64 = f64::max(end - secs_in_min, 0f64);
            
            let start_min = f64::floor(start / secs_in_min);
            let start_sec = start % secs_in_min;

            let end_min = f64::floor(end / secs_in_min);
            let end_sec = end % secs_in_min;

            let zero_cap = 9 as f64;

            let mut min_zero_start = "0";
            if start_min > zero_cap { min_zero_start = ""; }
            let mut sec_zero_start = "0";
            if start_sec > zero_cap { sec_zero_start = ""; }

            let mut min_zero_end = "0";
            if end_min > zero_cap { min_zero_end = ""; }
            let mut sec_zero_end = "0";
            if end_sec > zero_cap { sec_zero_end = ""; }

            println!("{}{}:{}{} - {}{}:{}{}", 
                min_zero_start, start_min, sec_zero_start, start_sec,
                min_zero_end, end_min, sec_zero_end, end_sec);
        }
    });
    
    inputbot::handle_input_events();
}