#!/bin/bash
# Input filename keyword and keyword for grep. Find the files in current dir, search the numbers needed, calculate sum and average

grep_keyword="total command "

usage() {
    cat << EOM
usage: $(basename "$0") [OPTION]...
    -f Input file name keyword
    -k Keyword to be searched in the input files
EOM
    exit 1
}

process_args() {
    while getopts ":f:k:h" option; do
        case "${option}" in
            f) filename_keyword=${OPTARG};;
            k) grep_keyword=${OPTARG};;
            h) usage;;
            *) echo "Invalid option: -${OPTARG}" >&2
            usage
            ;;
        esac
    done
}


# Find all files containing the filename keyword
files=$(find . -type f -name "*${filename_keyword}*"| cut -d/ -f2)

total_sum=0
total_count=0
total_aver=0

# Loop through the found files
for file in $files; do
    sum=0
    count=0

    tag=$(echo "$file" | cut -d- -f2)
    tmpfile="tmp_output_"$tag
    touch "$tmpfile"

    # Grep the keyword and extract the number. The number is the 2nd last word in the line.
    grep -i "${grep_keyword}" "$file" | awk '{print $(NF-1)}' > "$tmpfile"

    while IFS= read -r line; do
        # Add the number to sum
        sum=$(echo "$sum + $line" | bc)
        # Increment the count
        count=$((count + 1))
    done < "$tmpfile"

    # Calculate the average
    if [ $count -ne 0 ]; then
        average=$(echo "scale=6; $sum / $count" | bc)
        echo "Output file: $file"
        echo "Number file: $tmpfile"
        echo "Count: $count"
        echo "Sum: $sum"
        echo "Average: $average"
    else
        echo "The file is empty"
    fi
    total_sum=$(echo "$sum + $total_sum" | bc)
    total_count=$(echo "$count + $total_count" | bc)
done

echo "Total Sum: $total_sum"
echo "Total count: $total_count"
total_aver=$(echo "scale=6; $total_sum / $total_count" | bc)
echo "Total average: $total_aver"
