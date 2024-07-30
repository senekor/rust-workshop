#!/bin/bash
set -euxo pipefail

# Deploy the lab instructions to GitHub Pages.
# This script is normally run by GitHub Actions.

git fetch ; git checkout gh-pages ; git reset --hard origin/main

repo_path="$(git rev-parse --show-toplevel)"
cd "$repo_path"

# GitHub Pages expects the static website to be in the docs folder
rm -rf docs
mkdir docs

shopt -s dotglob
cp slides/dist_append/* docs/

cd slides
npm install

for idx in $(seq 1 6)
do
  npm run slidev -- build --base "/rust-workshop-extra/$idx/" "day/$idx/index.md"
  cp -r "day/$idx/dist" "../docs/$idx"
done

git config --global user.email "gh-pages@invalid.local"
git config --global user.name "gh-pages bot"

git add --all ; git commit --message "deploy" ; git push --force
