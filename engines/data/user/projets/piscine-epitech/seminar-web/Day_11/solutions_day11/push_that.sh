#!/bin/bash

if [ $# -eq 0 ]; then
    echo "Usage: $0 <commit_message>"
    exit 1
fi

git add .
git commit -m "$1"
git push || git push --set-upstream origin main || git push --set-upstream origin master