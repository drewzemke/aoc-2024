use common::puzzle::PuzzlePart;

pub struct Puzzle09a {}

impl PuzzlePart for Puzzle09a {
    fn description() -> &'static str {
        "Compute the 'checksum' of a filesystem after compacting it"
    }

    fn solve(input: &str) -> String {
        // even index entries of `input` are sizes of blocks, odd entries are sizes of spaces
        let input = input
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();

        // we're going to compute this checksum without ever actually constructing
        // the compacted filesystem. just a bunch of counting and careful bookkeeping.
        let mut checksum: usize = 0;

        // left cursor moves left to right one entry at a time, alternating between chunks
        // of file blocks and chunks of spaces
        let mut left_cursor = 0;

        // right cursor moves right to left, but only over the file chunks (it skips space chunks)
        let mut right_cursor = input.len() - 1; // even since length is odd

        // whenever the left cursor is on a space chunk, we'll look at the file chunk under the right cursor
        // and use it to "fill" the space. but that process doesn't always exhaust the blocks in the
        // right chunk, so we need to keep track of that
        let mut right_blocks_rem = input[right_cursor];

        // keeps track of what position in the compacted filesystem we would be at if we were
        // actually constructing it
        let mut pos = 0;

        // process until the left and right cursor meet
        while left_cursor < right_cursor {
            let chunk_size = input[left_cursor];

            if left_cursor % 2 == 0 {
                // file chunk, add the blocks to the checksum, increasing
                // the position marker between each addition
                let file_id = left_cursor / 2;

                // for each block of the chunk, add (id * position)
                // TODO: sum directly for a slight speed increase
                for i in pos..pos + chunk_size {
                    // println!("left cursor: adding {block_id} * {i}");
                    checksum += i * file_id;
                }
                pos += chunk_size;
            } else {
                // space chunk, fill it up from the file chunk under the right cursor
                for _ in 0..chunk_size {
                    // take one block from the right, moving the right cursor
                    // to the next block (to the left) if necessary
                    while right_blocks_rem == 0 {
                        right_cursor -= 2;
                        right_blocks_rem = input[right_cursor];
                    }

                    // need to break the loop if doing the above made the left
                    // and right cursors meet up
                    if right_cursor < left_cursor {
                        continue;
                    }

                    let file_id = right_cursor / 2;

                    // println!("right cursor: adding {block_id} * {next_pos}");
                    checksum += pos * file_id;

                    right_blocks_rem -= 1;
                    pos += 1;
                }
            }

            left_cursor += 1;
        }

        // lastly, add any remaining right cursor blocks to the checksum
        if left_cursor == right_cursor {
            let file_id = right_cursor / 2;

            for _ in 0..right_blocks_rem {
                // println!("finally: adding {block_id} * {next_pos}");
                checksum += pos * file_id;
                pos += 1;
            }
        }

        checksum.to_string()
    }
}
