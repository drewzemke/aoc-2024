use common::puzzle::PuzzlePart;

pub struct Puzzle09b {}

impl PuzzlePart for Puzzle09b {
    fn description() -> &'static str {
        "Compute the 'checksum' of a filesystem after compacting it, moving only whole files when possible"
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

        // again, we're gonna do this without actually constructing the compacted filesystem. we ride!
        let mut checksum: usize = 0;

        // moves left to right one entry at a time, alternating between files and spaces
        let mut cursor = 0;

        // this time, since we aren't always moving the files directly under the right cursor,
        // there's no point in having a right cursor. instead, we need to keep track of which
        // files we've moved. we'll store their indices in the input
        let mut moved_files = vec![];

        // keeps track of what position in the compacted filesystem we would be at if we were
        // actually constructing it
        let mut pos = 0;

        while cursor < input.len() {
            let chunk_size = input[cursor];

            if cursor % 2 == 0 {
                // file chunk: add the blocks to the checksum, increasing
                // the position marker between each addition

                // ... but only if this file hasn't already been moved.
                if moved_files.contains(&cursor) {
                    pos += chunk_size;
                    cursor += 1;
                    continue;
                }

                let file_id = cursor / 2;

                // for each block of the chunk, add (id * position)
                for i in pos..pos + chunk_size {
                    // println!("file chunk: adding {file_id} * {i}");
                    checksum += file_id * i;
                }
                pos += chunk_size;
            } else {
                // space chunk: we need to search leftwards from the right cursor to find the first
                // file that could fit here. keep doing that until we either fill the space or
                // we have a space too small to fit any of the unmoved files
                let mut space_remaining = chunk_size;

                // search for small-enough files from the right, skipping already-moved files
                let mut search_cursor = input.len() - 1;

                while space_remaining > 0 && search_cursor > cursor {
                    let file_size = input[search_cursor];

                    if file_size <= space_remaining && !moved_files.contains(&search_cursor) {
                        // "move" the file blocks here, meaning we add to the checksum
                        // and update the amount of remaining space to fill in this chunk
                        let file_id = search_cursor / 2;

                        for i in pos..pos + file_size {
                            // println!("space chunk: adding {file_id} * {i}");
                            checksum += file_id * i;
                        }
                        pos += file_size;

                        moved_files.push(search_cursor);
                        space_remaining -= file_size;
                    } else {
                        search_cursor -= 2;
                    }
                }

                // we need to skip the remaining spaces in our checksum computation,
                // since they won't be filled by any file
                if space_remaining > 0 {
                    pos += space_remaining;
                }
            }

            cursor += 1;
        }

        checksum.to_string()
    }
}
