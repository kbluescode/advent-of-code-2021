#!/usr/bin/env fish

set day_num $argv[1]

if test -z "$day_num"
    echo "day can't be empty"
    exit 1
end

set folder_name (printf "aoc-day%02d" $day_num)

if test -d $folder_name
    echo "$folder_name already exists!"
    exit 1
end

echo "creating $folder_name..."
cargo new $folder_name
or echo "failed to create $folder_name"

rm -rf $folder_name/.git
