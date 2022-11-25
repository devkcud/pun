#!/bin/bash

REPO_URL="https://github.com/devkcud/pun.git"
TEMP_FOLDER="/tmp/pun/"
BINARY_FOLDER="/usr/local/bin"

## Creating temporary folder
echo "Creating '$TEMP_FOLDER'."

if [ -d "$TEMP_FOLDER" ]; then rm -rf $TEMP_FOLDER; fi
mkdir $TEMP_FOLDER

## Cloning repo
echo "Cloning repo from '$REPO_URL'"
git clone $REPO_URL $TEMP_FOLDER -q
cd $TEMP_FOLDER || exit 1

## Building
echo "Running cargo build."
cargo build --release --quiet

## Copying binary file
echo "Copying to '$BINARY_FOLDER'"
sudo cp ./target/release/pun $BINARY_FOLDER/pun

# Removing temporary folder
echo "Removing '$TEMP_FOLDER'"
if [ -d "$TEMP_FOLDER" ]; then rm -rf $TEMP_FOLDER; fi

echo "Done."

