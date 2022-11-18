#!/bin/bash

REPO_URL="https://gitlab.com/devkcud/pun.git"
TEMP_FOLDER="/tmp/pun_src"
BINARY_FOLDER="/usr/local/bin"

## Temporary folder creation
echo "Creating '$TEMP_FOLDER'."

if [ -d "$TEMP_FOLDER" ]; then
  rm -rf $TEMP_FOLDER
fi

mkdir $TEMP_FOLDER

## Cloning repo
echo "Cloning repo from '$REPO_URL'"
git clone $REPO_URL $TEMP_FOLDER/pun -q
cd $TEMP_FOLDER/pun || exit 1

## Building
echo "Running cargo build."
cargo build --release --quiet

## Copying binary file
echo "Copying to '$BINARY_FOLDER'"

if [ -f "$BINARY_FOLDER" ]; then
  sudo rm $BINARY_FOLDER/pun
fi

sudo cp ./target/release/pun $BINARY_FOLDER

echo "Done."

