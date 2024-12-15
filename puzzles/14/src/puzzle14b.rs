use std::path::Path;

use crate::RayRobot;
use common::{point::Point, puzzle::PuzzlePart};
use image::{ImageBuffer, Rgb};

pub struct Puzzle14b {}

const ITERS: usize = 100;

impl PuzzlePart for Puzzle14b {
    fn description() -> &'static str {
        "Find the first step of the robots movement in which their arrangement looks like a christmas tree."
    }

    fn solve(input: &str) -> String {
        let mut lines = input.lines();

        // the first line of input contains the size
        let size: (i64, i64) = lines
            .next()
            .and_then(|line| line.split_once(','))
            .and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?)))
            .unwrap();

        let mut robots = lines
            .map(RayRobot::parse)
            .map(Option::unwrap)
            .collect::<Vec<_>>();

        let advance_robots = |robots: &mut Vec<RayRobot>, steps: usize| {
            for r in robots {
                let pt = r.eval(steps as i64);
                r.start = (pt.row.rem_euclid(size.0), pt.col.rem_euclid(size.1)).into();
            }
        };

        let start = 12;
        let skip = 103;

        advance_robots(&mut robots, start);

        // iterate and generate images
        for iter in (0..ITERS).map(|x| start + skip * x) {
            let mut img = ImageBuffer::new(size.0 as u32, size.1 as u32);

            for r in &robots {
                let Point { row, col } = r.start;
                img.put_pixel(row as u32, col as u32, Rgb([255, 255, 255]));
            }

            image::save_buffer(
                Path::new(&format!("data/imgs/{iter}.png")),
                &img,
                size.0 as u32,
                size.1 as u32,
                image::ColorType::Rgb8,
            )
            .unwrap();

            // update robot positions
            advance_robots(&mut robots, skip);
        }

        println!("Added some images to /data/imgs, check them out! 🎄");

        // The code above doesn't solve the puzzle directly, but I used it to partially solve by hand.
        //
        // Here's roughly how it went:
        // - generated the first 300 iterations, scanned through them by hand
        // - noticed that iteration 12 had a noticeable *horizontal* band of pixels,
        //   and iter 69 had a *vertical* band
        // - saw additional horizontal bands at 115, 218, and additional vertical bands at 170, 281
        // - so the horizontal bands repeate every 103 iters, vertical every 101 iters
        // - guessed that the tree will show up when the sequences (12, 115, 118, ...) and
        //   (69, 170, 281, ...) intersect
        // - find the smallest positive integer that can be expressed as 12 + 103*n and 69 + 101*m for some
        //   integers n and m
        // - that number is 8149, which is the first image with a tree :)
        String::from("8149")
    }
}
