set shell := ["fish", "-c"]

# `part` can be 'a' or 'b' 
# `arg` can be 'e' for example input
#       or 'r' for release mode
run day part="" arg="":
    #!/usr/bin/env fish
    set day (printf "%02d" {{day}})
    test {{part}} && set part --part {{part}}
    test "{{arg}}" = "e" && set example --example
    test "{{arg}}" = "r" && set release --release
    cargo run --quiet --bin "puzzle$day" $release -- $part $example

@setup day:
    setup/setup.fish {{day}}

@get-example day:
    cd setup/get-example && \
    deno run -A get-example.ts 2024 {{day}}

@get-input day:
    setup/get-input/get-input.fish {{day}}
