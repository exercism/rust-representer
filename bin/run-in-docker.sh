#!/usr/bin/env sh
set -e

# Synopsis:
# Run the representer on a solution using the representer Docker image.
# The representer Docker image is built automatically.

# Arguments:
# $1: exercise slug
# $2: absolute path to solution folder
# $3: absolute path to output directory

# Output:
# Writes the test results to a results.json file in the passed-in output directory.
# The test results are formatted according to the specifications at https://github.com/exercism/docs/blob/main/building/tooling/representers/interface.md

# Example:
# ./bin/run-in-docker.sh two-fer /absolute/path/to/two-fer/solution/folder/ /absolute/path/to/output/directory/
# If arguments not provided, print usage and exit
if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ]; then
    echo "usage: run-in-docker.sh exercise-slug ./relative/path/to/solution/folder/ ./relative/path/to/output/directory/"
    exit 1
fi

# build docker image
docker build --rm --no-cache -t exercism/rust-representer .

# Create output directory if it doesn't exist
output_dir="$3"
mkdir -p "$output_dir"

# run image passing the arguments
docker run \
    --rm \
    --network none \
    --read-only \
    --mount type=bind,source="${PWD}/${2}",destination=/solution \
    --mount type=bind,source="${PWD}/${output_dir}",destination=/output \
    --mount type=tmpfs,destination=/tmp \
    exercism/rust-representer $1 /solution/ /output/