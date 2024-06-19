#!/bin/bash
set -euo pipefail

cargo binstall mdbook mdbook-admonish mdbook-catppuccin

(
  cd final_project
  mdbook-admonish install
  mdbook-catppuccin install
)
