#!/usr/bin/env python3

import time
import sys
import os
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler

from importlib import reload

import mdscript

last_compile_time = -1
file_path = ''
dest_path = ''

rust_mode = False
debug_mode = False

class Handler(FileSystemEventHandler):
    @staticmethod
    def on_any_event(event):
        # super simple debounce
        global last_compile_time
        if time.time() - last_compile_time < 0.5:
            return
        last_compile_time = time.time()

        print('recompiling...', end='')
        if rust_mode: # either call cargo or the pre-compiled binary
            if debug_mode: os.system(f'cargo run {file_path} {dest_path}')
            else: os.system(f'target/debug/mdscript {file_path} {dest_path}')
        else:
            if debug_mode: reload(mdscript) # reload if in debug mode
            mdscript.compile_file(file_path, dest_path)
        print('done')

if __name__ == '__main__':
    if '-r' in sys.argv[1:3] or '--rust' in sys.argv[1:3]:
        rust_mode = True
    if '-d' in sys.argv[1:3] or '--debug' in sys.argv[1:3]:
        debug_mode = True
    if len(sys.argv) != 2 + rust_mode + debug_mode:
        print('usage: mdwatch.py [--rust | -r] [--debug | -d] <file_path>')
        exit(1)

    # directory of the file to watch
    file_path = os.path.abspath(sys.argv[-1])
    dir_path = os.path.dirname(file_path)
    dest_path = dir_path + '/' + os.path.basename(file_path).split('.')[0] + '.html'
    print(f'Watching {dir_path}\n- press ctrl+c to exit\n')

    # if we're in rust mode, cd to the directory of this script to run cargo
    if rust_mode:
        os.chdir(os.path.dirname(os.path.abspath(__file__)))
        # if not debug_mode, build it once in advance
        if not debug_mode: os.system('cargo build --release')

    # recompile any time anything changes in the directory
    event_handler = Handler()
    observer = Observer()
    observer.schedule(event_handler, dir_path, recursive=True)
    event_handler.on_any_event(None) # trigger initial compile
    observer.start()

    # DEBUG: while in development, run a second observer to watch the directory of the build stuff
    if debug_mode:
        observer2 = Observer()
        observer2.schedule(event_handler, os.path.dirname(os.path.abspath(sys.argv[0])), recursive=True)
        observer2.start()
    # END DEBUG

    try:
        while True:
            time.sleep(1)
    except:
        observer.stop()
        print('\nstopping')

