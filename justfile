_default:
    just --list --unsorted

setup:
    ./dev/scripts/setup.sh

# render slides on a dev server
slides day:
    cd slides && npm run slidev -- --port 304{{day}} day/{{day}}/index.md

serve-paekli-rs:
    just paekli-rs/serve

zellij:
    zellij --layout dev/zellij/default.kdl

demo:
    #!/bin/bash
    set -euo pipefail

    # setup helix config
    cp ~/.config/helix/config.toml helix-demo-config.toml
    sd 'kanagawa' 'catppuccin_latte' helix-demo-config.toml
    
    cd demos
    demos=( $(ls) )
    choice="$(echo "${demos[@]}" | tr ' ' "\n" | fzf)"

    cd $choice
    alacritty --option 'font.size=18' --command zellij --layout ../../dev/zellij/demo.kdl
