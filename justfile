_default:
    just --list --unsorted

@_setup-slides:
    command -v npm > /dev/null || (echo "npm must be installed" && exit 1)
    cd slides && [ -d node_modules ] || npm install

clone-submodule:
    #!/bin/bash
    set -euo pipefail
    if which jj &> /dev/null ; then
        jj git clone --colocate git@github.com:senekor/rust-workshop
    else
        git clone git@github.com:senekor/rust-workshop
    fi

# render slides on a dev server
slides day: _setup-slides
    cd slides && npm run slidev -- --port 304{{day}} day/{{day}}/index.md

zellij:
    zellij --layout dev/zellij/default.kdl

_demo day name:
    #!/bin/bash
    set -euo pipefail
    cd demos/day_{{ day }}/{{ name }}
    alacritty --option 'font.size=18' --command zellij --layout ../../../dev/zellij/demo.kdl

demo-1-mutable_references:
    @just _demo "1" mutable_references
demo-1-destructors:
    @just _demo "1" destructors

demo-2-declare_mod:
    @just _demo "2" declare_mod
demo-2-error_handling:
    @just _demo "2" error_handling
demo-2-exhaustiveness:
    @just _demo "2" exhaustiveness

demo-3-adapters:
    @just _demo "3" adapters
demo-3-for_loops:
    #!/bin/bash
    set -euo pipefail
    cd demos/day_3/for_loops
    just zellij-window

demo-4-cargo_deny:
    @just _demo "4" cargo_deny
demo-4-divan:
    @just _demo "4" divan
demo-4-itertools:
    @just _demo "4" itertools
demo-4-serde:
    @just _demo "4" serde
demo-4-use_lib:
    @just _demo "4" use_lib
