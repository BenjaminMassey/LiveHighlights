use inputbot::KeybdKey::*;
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;
use std::process;

fn main() {

    if false {
        test_time_functions();
    }

    let filename = "output.txt";

    let initial = Instant::now();

    create_file(filename);

    F8Key.bind(move || {
        if LShiftKey.is_pressed() && LControlKey.is_pressed() {
            let end = initial.elapsed().as_secs() as f64;
            let start = f64::max(end - 60f64, 0f64);
            let result = duration_str(start, end);

            println!("{}", result);

            append_file(filename, result);
        }
    });

    F9Key.bind(move || {
        if LShiftKey.is_pressed() && LControlKey.is_pressed() {
            process::exit(0);
        }
    });
    
    inputbot::handle_input_events();
}

fn secs_to_time(time: f64) -> [f64; 3] {
    let hours = f64::floor(time / 3600f64);
    let minutes = f64::floor((time % 3600f64) / 60f64); 
    let seconds = (time % 3600f64) % 60f64;
    return [hours, minutes, seconds];
}

fn time_formatted(input: [f64; 3]) -> String {
    let mut output = Vec::new();
    for i in 0..input.len() {
        let zero = if input[i] > 9f64 { "" } else { "0" };
        output.push(format!("{}{}", zero, input[i]));
    }
    return format!("{}:{}:{}", output[0], output[1], output[2]);
}

fn duration_str(start: f64, end: f64) -> String {
    let start_tuple = secs_to_time(start as f64);
    let end_tuple = secs_to_time(end as f64);

    let start_str = time_formatted(start_tuple);
    let end_str = time_formatted(end_tuple);

    return format!("{} - {}", start_str, end_str);
}

fn create_file(filename: &str) {
    let _file = match File::create(filename) {
        Err(e) => panic!("{}", e),
        Ok(_file) => _file,
    };
}

fn append_file(filename: &str, text: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(filename)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", text) {
        eprintln!("{}", e);
    }
}

fn test_time_functions() {
    let test_values: Vec<f64> = vec![ 9f64, 61f64, 403f64, 8027f64, 481323f64 ];
    for val in test_values.iter() {
        println!("{} seconds -> \"{}\"", val, time_formatted(secs_to_time(*val)));
    }
}