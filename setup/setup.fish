#!/usr/bin/env fish

function usage
    echo "Usage: setup/setup.fish <day_number>"
end

function done
    echo " done."
end

if test (count $argv) -ne 1
    usage
    exit 1
end

# pad the input day number with a zero
set day $argv[1]
set day_padded (printf "%02d" $day)

set template_dir "setup/template"
set puzzle_dir "puzzles/$day_padded"

echo "Setting up puzzle for "(set_color -o green)"Day $day"(set_color normal)

# copy template directory
if test -d $puzzle_dir
    echo "Error: $puzzle_dir already exists"
    exit 1
end
cp -r $template_dir $puzzle_dir

echo -n "• Creating "(set_color blue)"directory "(set_color -i cyan)"$puzzle_dir"(set_color normal)" from template..."
# process files: replace DAYNUM with the padded day number
for file in $puzzle_dir/**
    if test -f $file
        # replace content in files (differs based on OS)
        if test (uname) = "Darwin"
            sed -i '' "s/DAYNUM/$day_padded/g" $file
        else
            sed -i "s/DAYNUM/$day_padded/g" $file
        end

        # rename files
        set new_name (echo $file | sed "s/DAYNUM/$day_padded/g")
        if test "$file" != "$new_name"
            mv $file $new_name
        end
    end
end
done

# get the input and examples
set data_dir $puzzle_dir/data
mkdir $data_dir

echo -n "• Downloading "(set_color blue)"puzzle input"(set_color normal)" to "(set_color -i cyan)"$data_dir/input..."(set_color normal)
just get-input $day > $data_dir/input; or begin
    echo "Error: Could not get puzzle input." 1>&2
    exit 1
end
done

echo -n "• Downloading "(set_color blue)"puzzle example"(set_color normal)" to "(set_color -i cyan)"$data_dir/example..."(set_color normal)
just get-example $day > $data_dir/example; or begin
    echo "Error: Could not get puzzle example." 1>&2
    exit 1
end
done

# boom, done
echo (set_color -o magenta)"Good luck!"(set_color normal)

