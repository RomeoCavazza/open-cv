#!/bin/bash
# Script pour afficher toutes les intros (pitch) des resume.json
find data/instances -name resume.json | while read file; do
  intro=$(jq -r '.profile.pitch' "$file")
  echo "${file}:"
  echo "$intro"
  echo "---"
done
