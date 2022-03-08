extern crate core;

use std::env;
use std::fs::File;
use std::io::Read;
use json;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args.last().unwrap());
    let mut contents = String::new();
    file.unwrap().read_to_string(&mut contents);
    let input = json::parse(&contents).unwrap();
    let output_args = input["output_args"].as_str().unwrap();
    let clips = &input["clips"];
    for clip in clips.members() {
        let input_file = clip["input_filename"].as_str().unwrap();
        let output_file = clip["output_filename"].as_str().unwrap();
        let start_time = clip["start_timestamp"].as_number().unwrap();
        let end_time = clip["end_timestamp"].as_number().unwrap();
        let command = format!("-loglevel quiet -i {input_file} -ss {start_time} -to {end_time} {output_args} {output_file}");
        println!("ffmpeg {}", command);
        let output =process::Command::new("ffmpeg").args(command.split(" ")).spawn();
        println!("{}", output.unwrap().wait().unwrap())
    }
}
