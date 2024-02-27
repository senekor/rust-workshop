#!/bin/bash
set -euo pipefail

repo_path="$(git rev-parse --show-toplevel)"
cd "$repo_path"

cd final_project
mdbook-admonish install
mdbook-catppuccin install
mdbook build
