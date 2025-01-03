set shell := ["fish", "-c"]

# `arg1` can be:
#   - 'a' or 'b' (part a or part b)
#   - 'e'        (use example input)
#   - 'r'        (run in release mode)
# `arg2` can be 'e' or 'r' (same as above)
run day arg1="" arg2="":
    #!/usr/bin/env fish
    set day (printf "%02d" {{day}})
    test "{{arg1}}" = "a" -o "{{arg1}}" = "b" && set part --part {{arg1}}
    test "{{arg1}}" = "e" -o "{{arg2}}" = "e" && set example --example
    test "{{arg1}}" = "r" -o "{{arg2}}" = "r" && set release --release
    cargo run --quiet --bin "puzzle$day" $release -- $part $example

@setup day:
    setup/setup.fish {{day}}

@get-example day:
    cd setup/get-example && \
    deno run -A get-example.ts 2024 {{day}}

@get-input day:
    setup/get-input/get-input.fish {{day}}

@get-all-inputs:
    #!/usr/bin/env fish
    for N in (seq 1 9); just get-input $N > ./puzzles/0$N/data/input; end
    for N in (seq 10 25); just get-input $N > ./puzzles/$N/data/input; end
