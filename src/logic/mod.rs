use image::{GenericImageView, Rgba};

use std::fs;
use std::io;
use std::path::Path;

fn diff_rgba3(rgba1: Rgba<u8>, rgba2: Rgba<u8>) -> i32 {
    (rgba1[0] as i32 - rgba2[0] as i32).abs()
        + (rgba1[1] as i32 - rgba2[1] as i32).abs()
        + (rgba1[2] as i32 - rgba2[2] as i32).abs()
}

fn get_succ(prev: usize, percents: &[Vec<f64>], used: &[bool]) -> (usize, f64) {
    let default = percents[prev][prev];
    percents[prev]
        .iter()
        .enumerate()
        .fold((prev, default), |acc, (i, elt)| {
            if i == prev || used[i] {
                acc
            } else if *elt < acc.1 {
                (i, *elt)
            } else {
                acc
            }
        })
}

fn fill_percents(
    frames_count: usize,
    images: &[image::DynamicImage],
    pixels: &[Vec<Rgba<u8>>],
) -> Vec<Vec<f64>> {
    let mut percents: Vec<Vec<f64>> =
        vec![vec![std::f64::MAX; frames_count]; frames_count];

    for (i1, img1) in images.iter().enumerate() {
        for (i2, _) in images.iter().skip(i1 + 1).enumerate() {
            let idx2 = i2 + i1 + 1;

            let pix1 = &pixels[i1];
            let pix2 = &pixels[idx2];
            let mut accum = 0;
            let zipper = pix1.iter().zip(pix2.iter());
            for (pixel1, pixel2) in zipper {
                accum += diff_rgba3(*pixel1, *pixel2);
            }

            let percent = accum as f64 * 100.0
                / (255.0 * 3.0 * (img1.width() * img1.height()) as f64);

            percents[i1][idx2] = percent;
            percents[idx2][i1] = percent;
        }
    }

    percents
}

fn copy_images(sequence: &[usize], paths: &[&str], output_location: &str) {
    for (i, img_id) in sequence.iter().enumerate() {
        fs::copy(&paths[*img_id], format!("{}/{:09}.png", output_location, i))
            .unwrap();
    }
}

fn build_sequence(
    start: usize,
    used: &mut Vec<bool>,
    percents: &[Vec<f64>],
    threshold: f64,
) -> Vec<usize> {
    let mut sequence = vec![start];
    used[start] = true;

    loop {
        let prev = sequence[sequence.len() - 1];
        let (succ, min_coef) = get_succ(prev, &percents, &used);

        if min_coef > threshold {
            break;
        }

        sequence.push(succ);
        used[succ] = true;
    }

    loop {
        let prev = sequence[0];
        let (succ, min_coef) = get_succ(prev, &percents, &used);

        if min_coef > threshold {
            break;
        }

        sequence.insert(0, succ);
        used[succ] = true;
    }

    sequence
}

fn get_biggest_sequence(sequences: &[Vec<usize>]) -> Vec<usize> {
    let (position_longest, _) =
        sequences
            .iter()
            .enumerate()
            .fold((0, 0), |(pos, len), (i, elt)| {
                if elt.len() > len {
                    (i, elt.len())
                } else {
                    (pos, len)
                }
            });

    sequences[position_longest].clone()
}

pub fn unshuffle_video(
    input_frames: &Path,
    threshold: f64,
    output_frames: &str,
    is_rev: bool,
) -> io::Result<()> {
    let mut paths: Vec<String> = Vec::new();

    if input_frames.is_dir() {
        for entry in fs::read_dir(input_frames)? {
            let entry = entry?;
            let filename = entry.path().into_os_string().into_string().unwrap();
            paths.push(filename);
        }
    }

    let frames_count = paths.len();

    paths.sort_unstable();
    let paths: Vec<&str> = paths.iter().map(|p| p.as_str()).collect();

    let images: Vec<image::DynamicImage> = paths
        .iter()
        .map(|p| image::open(p).expect("Unable to load image."))
        .collect();

    let pixels: Vec<Vec<Rgba<u8>>> = images
        .iter()
        .map(|i| i.pixels().map(|p| p.2).collect())
        .collect();

    let percents = fill_percents(frames_count, &images, &pixels);

    let mut used = vec![false; frames_count];
    let mut sequences = Vec::<Vec<usize>>::new();
    while let Some(start) = used.iter().position(|elt| !elt) {
        let sequence = build_sequence(start, &mut used, &percents, threshold);
        sequences.push(sequence);
    }

    let mut biggest = get_biggest_sequence(&sequences);

    if is_rev {
        biggest.reverse();
    }

    copy_images(&biggest, &paths, output_frames);

    Ok(())
}
