use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::process;
use std::time::Instant;

use inputbot::KeybdKey::*;

fn main() {
    let filename = "output.txt";

    let initial = Instant::now();

    create_file(filename);

    F8Key.bind(move || {
        if LShiftKey.is_pressed() && LControlKey.is_pressed() {
            let end = initial.elapsed().as_secs() as i32;
            let start = i32::max(end - 60, 0);
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

fn secs_to_time(time: i32) -> [i32; 3] {
    let hours = time / 3600;
    let minutes = (time % 3600) / 60;
    let seconds = (time % 3600) % 60;
    [hours, minutes, seconds]
}

fn time_formatted(input: [i32; 3]) -> String {
    let mut output = ["".to_string(), "".to_string(), "".to_string()];
    for i in 0..input.len() {
        let zero = if input[i] > 9 { "" } else { "0" };
        output[i] = format!("{}{}", zero, input[i]);
    }
    format!("{}:{}:{}", output[0], output[1], output[2])
}

fn duration_str(start: i32, end: i32) -> String {
    let start_tuple = secs_to_time(start);
    let end_tuple = secs_to_time(end);

    let start_str = time_formatted(start_tuple);
    let end_str = time_formatted(end_tuple);

    format!("{} - {}", start_str, end_str)
}

fn create_file(filename: &str) {
    let _file = match File::create(filename) {
        Err(e) => panic!("{}", e),
        Ok(_file) => _file,
    };
}

fn append_file(filename: &str, text: String) {
    let mut file = OpenOptions::new().append(true).open(filename).unwrap();

    if let Err(e) = writeln!(file, "{}", text) {
        eprintln!("{}", e);
    }
}

#[test]
fn test_time_functions() {
    let test_values = [
        (9, "00:00:09"),
        (61, "00:01:01"),
        (403, "00:06:43"),
        (8027, "02:13:47"),
        (481323, "133:42:03"),
    ];
    for (secs, time) in test_values.iter() {
        assert_eq!(*time, time_formatted(secs_to_time(*secs)));
    }
}
