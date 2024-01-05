_default:
    just --list --unsorted

@_setup-slides:
    command -v npm > /dev/null || (echo "npm must be installed" && exit 1)
    cd slides && [ -d node_modules ] || npm install

# render slides on a dev server
slides day: _setup-slides
    cd slides && npm run slidev -- day/{{day}}/index.md
