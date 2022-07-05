# wip, do not touch

quick tool to parse something approximating markdown into fairly nice looking html files for my school-notes. I've written this in rust. I don't know rust, so it's 95% guesswork and rewriting stuff until it compiles - total nightmare :)

# mdwatch.sh

This is a shell script that you can call on a markdown file to recompile it everytime it gets saved. Check the script for its dependencies and usage.

You can symlink it to somewhere on $PATH if you want, like so:
```bash
# in dir where you cloned mdscript:
chmod +x mdwatch.sh # make mdwatch executable
ln -s /path/to/mdwatch.sh ~/.local/bin/mdwatch # run as mdwatch without path to script location
```
