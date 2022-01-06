#!/usr/bin/env python3

import time
import sys
import os
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler

import mdscript

last_compile_time = -1
file_path = ''
dest_path = ''

class Handler(FileSystemEventHandler):
    @staticmethod
    def on_any_event(event):
        global last_compile_time
        if time.time() - last_compile_time < 0.5: # super simple debounce
            return
        last_compile_time = time.time()
        print('recompiling...', end='')
        mdscript.compile_file(file_path, dest_path)
        print('done')

if __name__ == '__main__':
    if len(sys.argv) != 2:
        print('Usage: python3 mdwatch.py <path>')
        sys.exit(1)

    # directory of the file to watch
    file_path = os.path.abspath(sys.argv[1])
    dir_path = os.path.dirname(file_path)
    dest_path = dir_path + '/' + os.path.basename(file_path).split('.')[0] + '.html'
    print(f'Watching {dir_path}\n- press ctrl+c to exit\n')

    # recompile any time anything changes in the directory
    event_handler = Handler()
    observer = Observer()
    observer.schedule(event_handler, dir_path, recursive=True)
    event_handler.on_any_event(None) # trigger initial compile
    observer.start()

    # DEBUG: while in development, run a second observer to watch the directory of the build stuff
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

