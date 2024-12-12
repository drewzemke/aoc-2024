#!/usr/bin/env fish

set part $argv[1]
set file data/soln-$part

if not test -e $file
    # no need to make a scene
    exit 0
end

if test (uname) = "Darwin"
    # macos
    cat $file | pbcopy
else
    # linux
    cat $file | xclip -selection clipboard
end

