extern crate image;

pub mod logic;

use std::env;
use std::fs;
use std::path::Path;
use std::process::{exit, Command};
use std::time::Instant;

pub use logic::unshuffle_video;

fn empty_dir(path_str: &str) -> std::io::Result<()> {
    let path = Path::new(path_str);
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            fs::remove_file(entry.path())?;
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 6 {
        panic!("Bad input parameters: exec_file video_input fps similarity_threshold is_rev video_output");
    }

    let video_path = &args[1];
    let fps = &args[2];
    let threshold: f64 = (&args[3]).parse().unwrap();
    let is_rev = &args[4] == "true";
    let output_name = &args[5];

    fs::create_dir_all("./frames")
        .expect("Unable to create temporary directory frames.");
    fs::create_dir_all("./output_frames")
        .expect("Unable to create temporary directory output frames.");

    let _ = empty_dir("./frames");
    let _ = empty_dir("./output_frames");

    Command::new("ffmpeg")
        .args(&["-r", "1", "-i", video_path, "-r", "1", "./frames/%09d.png"])
        .status()
        .expect("Unable to run cmd to retrieve frames.");

    let start = Instant::now();

    let input = Path::new("./frames");
    let output = "./output_frames";
    let _ = unshuffle_video(&input, threshold, &output, is_rev);

    println!("Elapsed time: {}us.", start.elapsed().as_micros() as u64);

    Command::new("ffmpeg")
        .args(&[
            "-r",
            fps,
            "-i",
            format!("{}/%09d.png", output).as_str(),
            "-vf",
            format!("fps={}", fps).as_str(),
            "-pix_fmt",
            "yuv420p",
            output_name,
        ])
        .status()
        .expect("Unable to run cmd to build video.");

    exit(0);
}
