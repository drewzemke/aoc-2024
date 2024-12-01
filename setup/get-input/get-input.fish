#!/usr/bin/env fish

if test (count $argv) -eq 0
    echo "Error: Please provide a day number as argument" 1>&2
    exit 1
end

set cookie (cat setup/get-input/.session-cookie 2>/dev/null; or begin
    echo "Error: Cookie file not found at setup/get-input/.session-cookie" 1>&2
    exit 1
end)

curl -s -H "Cookie: $cookie" https://adventofcode.com/2024/day/$argv[1]/input
