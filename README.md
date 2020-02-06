# Unshuffle Video

Given a shuffled video with foreign images, this program removes foreign images
and rebuild the video.

## Version

- rustc: 1.39.0
- rustup: 1.20.2
- cargo: 1.39.0
- rustfmt: 1.4.8-stable
- ffmpeg: 4.2.2
- Ubuntu: 18.04.3 lts

## Installation

First you have to install:
- ffmpeg: https://www.ffmpeg.org/download.html

Then you have to install:
- Rust: https://www.rust-lang.org/tools/install

Once this is done, you can clone this project:
- `git clone https://github.com/lakwet/unshuffle_video.git`

## How to use it

This a program, not a library, so you have to build it and then use its executable.

### The config file (optional)

Rust can use SIMD if an environment variable is set.

You can set this variable by doing this at the project's root: `source config`.

### Build the project and use it

To build the project there is a _Makefile_.

You can just do: `make build`.

The executable is here: `./target/release/unshufflevideo`

This executable requires 5 arguments:
- the path to the input video (mp4 only)
- video fps (in the example video it is 25)
- a threshold (see below) which is a float
- a boolean (true or false) (see below)
- the path for the output video

So at the project root you can run:
`./target/release/unshufflevideo video_input_path.mp4 25 5.5 true video_output.mp4`

### The threshold

To unshuffle a video, this program compares frames to each other. This comparison
returns a difference percentage.

Two very similar images will have a very small difference percentage.

The threshold is the percentage at which we consider two images are too different
to keep them is the same sequence. If the difference is too big, then there is
a foreign image.

In the example video, a good threshold is *5.5* percent. With other video it might
be another value.

### The boolean

When the program aggregates images to rebuild the video, it can aggregates
images in an order or in the reverse order.

This variable allows to reverse the output video.

In the example video, the default output video is running backward. So we have
to reverse images order.

If set to *true*, it reverses the output video order.

## Algorithms

The main idea is that two images in a video are closed to each other if they
have a very high similarity.

For this purpose several similarity algorithms have been tested:
- dssim ([https://crates.io/crates/dssim](https://crates.io/crates/dssim)), but
the Rust lib was slow and results were not good, despite that the video has
almost been rebuilt.
- [https://crates.io/crates/simimgrs](https://crates.io/crates/simimgrs), but it
returns only 0/1, so it was unusable to discriminate similarities between
images.
- dhash ([https://crates.io/crates/dhash](https://crates.io/crates/dhash)), but
the _distance_ was not accurate enough to discriminate similarities between
images.

And then a percentage computation was found on [https://rosettacode.org/wiki/Percentage_difference_between_images](https://rosettacode.org/wiki/Percentage_difference_between_images) and this time, it worked very well. Moreover
the code was fast. By improving the code, it was possible to make it even faster
(with the _dssim_, it was more than 10 minutes).

So, the idea is to start with any image and to append the most similar image to the
front or the back of the sequence. That way the sequence grows until there is no
similar image left.

It is redone until all images have been used.

At the end, there are several sequences.

The longest sequence is used to rebuild the video.

## Engineering

I decided to code this project in Rust as a training exercice. I was curious to know
what was possible to do with Rust on video or image.

Rust is only 10 years old, and there are not as many libraries as in other
language (such as C++). There are already a lot of bindings (FFI) to C++ but
most of the time crates are not documented.

From a set of images, this program can rebuild the longest images sequence.
However this program can only reorder images.

To split the video into frames, or from frames to rebuild a video, another
program is used: *ffmpeg*.

It is more convenient if this program wraps all the dirty work (i.e: split the
video into frames, from frames rebuild the video). For this purpose I just
use the `Command` function from Rust to call the *ffmpeg* program.

## Nota Bene

With the example video, it takes about 14s to rebuild the video on a intel i5
with 16GB RAM.
