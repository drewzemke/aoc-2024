@get-example day:
    cd ./get-example && \
    deno run -A main.ts 2023 {{day}}

@get-input day:
    ./get-input/main.fish {{day}}
