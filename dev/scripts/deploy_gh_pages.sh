#!/bin/bash
set -eo pipefail

cd "$(git rev-parse --show-toplevel)"

if [ -n "$(git status --porcelain)" ] ; then
    echo "dirty working tree, aborting"
    exit 1
fi

git checkout main
git pull
git checkout gh-pages
git reset --hard main

cd final_project
mdbook-admonish install
mdbook-catppuccin install
mdbook build --dest-dir ../docs

git add --all
git commit --message "Deploy 'Final Project' book to GitHub Pages"
git push --force

git checkout main
