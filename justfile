@setup day:
    setup/setup.fish {{day}}

@get-example day:
    cd setup/get-example && \
    deno run -A get-example.ts 2023 {{day}}

@get-input day:
    setup/get-input/get-input.fish {{day}}
