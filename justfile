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
    demos=(
        mutable_references
        destructors
        declare_mod
        error_handling
        exhaustiveness
        adapters
        for_loops
        cargo_deny
        divan
        itertools
        serde
        use_lib
    )
    choice="$(echo "${demos[@]}" | tr ' ' "\n" | fzf)"

    function run_demo() {
        cd demos/day_${1}/${2}
        alacritty --option 'font.size=18' --command zellij --layout ../../../dev/zellij/demo.kdl
    }
    case "$choice" in
        mutable_references) run_demo "1" mutable_references ;;
        destructors)        run_demo "1" destructors ;;

        declare_mod)        run_demo "2" declare_mod ;;
        error_handling)     run_demo "2" error_handling ;;
        exhaustiveness)     run_demo "2" exhaustiveness ;;

        adapters)           run_demo "3" adapters ;;
        for_loops)          cd demos/day_3/for_loops ; just zellij-window ;;

        cargo_deny)         run_demo "4" cargo_deny ;;
        divan)              run_demo "4" divan ;;
        itertools)          run_demo "4" itertools ;;
        serde)              run_demo "4" serde ;;
        use_lib)            run_demo "4" use_lib ;;
    esac
