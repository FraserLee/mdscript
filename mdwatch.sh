#!/usr/bin/env bash

# DEPENDENCIES:
# - mdscript on $PATH for non-debug mode
# - entr for markdown file watching
# - cargo-watch for mdscript source watching
# - shell stuff (GNU coreutils, bash, etc)

usage () {
  echo "usage: mdwatch.sh [--debug | -d] <file_path>" 1>&2
  exit 1
}

# check for valid number of args (1 | 2)
[[ $# -ne 1 && $# -ne 2 ]] && usage

# last argument is the file we're watching
FILE=${!#}
# if file we're watching is not in fact a file, print usage
[[ ! -f $FILE ]] && usage

FILE_PATH=$( realpath "$FILE" )
DIR_PATH=$( dirname "$FILE_PATH" )
DEST_PATH="$DIR_PATH/$( basename -s md "$FILE_PATH" )html"

if [[ $# -eq 1 ]]; then # if one arg, not in debug mode
  # if not in debug mode, watch md file with $PATH mdscript
  echo "Watching $DIR_PATH"
  echo "- press ctrl-c to exit"
  echo "$FILE_PATH" | entr mdscript /_ "$DEST_PATH"
elif [[ "$1" == "-d" || "$1" == "--debug" ]]; then # if two args and debug flag passed, enter debug mode
  CARGO_DIR=$(dirname -- "$( realpath "${BASH_SOURCE[0]}" )")
  echo "Debug mode"
  echo "Markdown file: $FILE_PATH"
  echo "Mdscript dir: $CARGO_DIR"
  echo "- press ctrl-c to exit"
  # debug mode: cd to directory of this script, following symlinks
  pushd "$CARGO_DIR" &> /dev/null
  # re-run debug build mdscript when we modify the source code
  cargo-watch -qx "run -- $FILE_PATH $DEST_PATH" &
  # use debug mdscript build to recompile markdown file
  echo "$FILE_PATH" | entr cargo run /_ "$DEST_PATH"
  popd &> /dev/null
else # two args, but first arg isn't a debug flag
  usage
fi

echo "stopping"
