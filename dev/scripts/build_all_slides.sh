#!/bin/bash
set -euo pipefail

repo_path="$(git rev-parse --show-toplevel)"
cd "$repo_path"

cd slides
npm install

for idx in $(seq 1 6)
do
    npm run slidev -- build "day/$idx/index.md"
done
