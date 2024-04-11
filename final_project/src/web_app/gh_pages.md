# GitHub Pages

GitHub Pages is a free static website hosting service.
It's very convenient to have a nice website for your GitHub projects.
However, if your project is precisely a website, GitHub Pages can be used as the actual deployment!

This section is mostly about how to write a GitHub Action to automate that deployment process.
It's not too much work, so let's get started.

## A new workflow for automatic deployment

First, we need a new workflow.
There should already be a couple in `.github/workflows`.
Let's create the file `.github/workflows/gh_pages.yml`.

Our workflow should run every time we push to the main branch:

```yml
name: GitHub Pages
on:
  push:
    branches: main
```

It's also gonna need write-access to our repository, so it can make a commit where the finished website will be stored.
Don't worry, that commit won't be polluting the main branch.

```yml
permissions:
  contents: write
```

We're gonna have a single job to run in the default Ubuntu environment:

```yml
jobs:
  pages:
    name: Deploy GitHub Pages
    runs-on: ubuntu-latest
```

Now we need to define the steps to run.
The first step is the same for almost all workflows: `uses: actions/checkout` to get access to the code of the repository itself.

```yml
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
```

Next up we'll use `Swatinem/rust-cache@v2` to cache our build.
This will speed up future builds quite a bit, because dependencies won't have to download and compile every time.

```yml
      - uses: actions/checkout@v4
      - uses: Swatinem/rust-cache@v2
```

The rest is basically just bash scripting:

```yml
      - uses: Swatinem/rust-cache@v2
      - run: |
          rustup target add wasm32-unknown-unknown
          cd final_project/paekli-web
          wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.19.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
          ./trunk build --release --public-url /rust-workshop
          mv dist ../../docs
          git config --global user.name "GitHub Actions Bot"
          git config --global user.email "bot@invalid.local"
          git checkout -b gh-pages
          git add ../../docs
          git commit --message "GitHub Pages Deployment"
          git push --force --set-upstream origin gh-pages
```

If you prefer, you can put this script in a regular `.sh` file in your repository so it can be tested more easily.

Let's explain a couple things that might not be obvious:

- `wasm32-unknown-unknown` is needed to compile Rust to WebAssembly.
- The `wget` command downloads a binary of `trunk` into the current directory.
- The `--public-url /rust-workshop` is necessary because our website is not located at the root path of the domain.
- We move the `dist` folder to `rust-workshop/docs` because that's where GitHub Pages expects our website to be located for deployment.
- The git configuration of username and email is irrelevant, these commits will be overwritten regularly.
- Lastly, we force push the built website to a branch called `gh-pages`.
  More on that next.

## Enabling GitHub Pages

1. go to GitHub
1. navigate to your repo
1. go to the "Settings" tab
1. click on "Pages" in the sidebar
1. under "Source", "Deploy from a branch" should already be selected
1. under "Branch", change "None" to "gh-actions" and "/ (root)" to "/docs"

And that should be it!
With this configuration, GitHub Pages will look inside the `/docs` directory of your repository on the `gh-actions` branch for a website to deploy.

```admonish success
Go to `YOUR_GH_USERNAME.github.io/rust-workshop` to enjoy the fruit of your labor!
```
