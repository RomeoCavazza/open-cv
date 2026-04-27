# FILE AND DIRECTORY LISTING PROGRAM

import os

def list_files_and_dirs(path):
    print(path + ":")
    
    # LIST FILES AND DIRECTORIES
    for item in os.listdir(path):
        print(item)
    print()

    # RECURSIVELY LIST SUBDIRECTORIES
    for item in os.listdir(path):
        full_path = os.path.join(path, item)
        if os.path.isdir(full_path):
            list_files_and_dirs(full_path)

# MAIN PROGRAM

current_dir = os.getcwd()
list_files_and_dirs(current_dir)