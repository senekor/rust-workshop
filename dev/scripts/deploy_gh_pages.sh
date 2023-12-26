#!/bin/bash
set -euo pipefail

# Deploy the slides to GitHub Pages.
# This script is normally run by GitHub Actions.

git checkout -b gh-pages ; git reset --hard origin/main

repo_path="$(git rev-parse --show-toplevel)"
cd "$repo_path"

rm -rf docs
mkdir -p docs

index="
# Rust Workshop Slides

- [Session 1](./1/index.html)
- [Session 2](./2/index.html)
- [Session 3](./3/index.html)
- [Session 4](./4/index.html)
- [Session 5](./5/index.html)
- [Session 6](./6/index.html)
"
echo "$index" > docs/index.md

cd slides
npm install

for idx in $(seq 1 6)
do
    vite_config="
export default {
  base: '/rust-workshop-extra/$idx'
}"
    vite_config_path="session/$idx/vite.config.js"
    echo "$vite_config" > "$vite_config_path"

    npm run slidev -- build --out "$repo_path/docs/$idx" "session/$idx/index.md"

    rm "$vite_config_path"
done

git config --global user.email "bot@example.local"
git config --global user.name "gh-pages bot"
git add --all ; git commit --message "Deploy slides to GitHub Pages"
git push --force --set-upstream origin gh-pages
