use inputbot::KeybdKey::*;
use std::{time::Instant, string};

fn main() {

    let initial = Instant::now();

    F8Key.bind(move || {
        if LShiftKey.is_pressed() && LControlKey.is_pressed() {
            let end = initial.elapsed().as_secs() as f64;
            let start = f64::max(end - 60f64, 0f64);
            
            let result = duration_str(start, end);

            println!("{}", result);
        }
    });
    
    inputbot::handle_input_events();
}

fn secs_to_mins_secs(time: f64) -> (f64, f64) {
    let secs_in_min = 60 as f64;
    let min = f64::floor(time / secs_in_min);
    let sec = time % secs_in_min;
    return (min, sec);
}

fn mins_secs_formatted(inp: (f64, f64)) -> String {
    let zero_cap = 9 as f64;
    let mut first_zero = "0";
    if inp.0 > zero_cap { first_zero = ""; }
    let mut second_zero = "0";
    if inp.1 > zero_cap { second_zero = ""; }
    return format!("{}{}:{}{}", 
                    first_zero, inp.0,
                    second_zero, inp.1);
}

fn duration_str(start: f64, end: f64) -> String {
    let start_tuple = secs_to_mins_secs(start as f64);
    let end_tuple = secs_to_mins_secs(end as f64);

    let start_str = mins_secs_formatted(start_tuple);
    let end_str = mins_secs_formatted(end_tuple);

    return format!("{} - {}", start_str, end_str);
}