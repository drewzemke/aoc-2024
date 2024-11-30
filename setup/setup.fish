#!/usr/bin/env fish

function usage
    echo "Usage: template/main.fish <day_number>"
end

if test (count $argv) -ne 1
    usage
    exit 1
end

# pad the input day number with a zero
set day_num (printf "%02d" $argv[1])

set template_dir "setup/template"
set puzzle_dir "puzzle$day_num"

# copy template directory
if test -d $puzzle_dir
    echo "Error: $puzzle_dir already exists"
    exit 1
end
cp -r $template_dir $puzzle_dir

# process files: replace DAYNUM with the padded day number
for file in $puzzle_dir/**
    if test -f $file
        # replace content in files
        sed -i '' "s/DAYNUM/$day_num/g" $file

        # rename files
        set new_name (echo $file | sed "s/DAYNUM/$day_num/g")
        if test "$file" != "$new_name"
            mv $file $new_name
        end
    end
end

echo "Created $puzzle_dir from template."

# TODO: download input and examples
