# Hello World

The most important project we'll be using for this component is [PyO3](https://pyo3.rs).
It does most of the heavy lifting for us so Python and Rust can talk to each other.
That means we have more time to write actual code.
Sweet!

Let's start by initializing a new package, sticking closely to PyO3's own guide.

```sh
cd final_project
mkdir paekli-py
cd paekli-py
python -m venv .venv
source .venv/bin/activate
```

The PyO3 project has a build tool called `maturin`:

```sh
pip install maturin
maturin init --bindings pyo3
```

This will generate a _Rust_ project with the necessary boilerplate to use it as a Python module.
You might be interested to take a look at `Cargo.toml`, to see what kind of package configuration is needed.

Next, take some time to understand the generated `src/lib.rs`.
You'll notice the annotations `#[pyfunction]` and `#[pymodule]` are used to make Rust code available to Python.
There are also some Python-specific types like `PyResult` and `PyModule`.

Let's try to call the generated `sum_as_string` function from Python.
The following command installs our Rust module into the virtual Python environment:

```sh
maturin develop
```

Now the module `paekli_py` should be available to import from Python:

```
> python
>>> import paekli_py
>>> paekli_py.sum_as_string(123, 678)
'801'
```

```admonish check title="Release ?"
Hurray!
You can now call Rust code from Python 🥳

For most components, we put effort in a continuous release workflow.
However, we won't do that for the Python extension module.

If you simply want to extend your own Python code with Rust, there is no loss.
If you do want to publish a Python module on PyPI, I encourage you to explore that on your own.

`maturin` actually generates a GitHub Actions workflow to automatically publish to PyPI.
It won't be active by default, because it's not at the root of the Git repository.
You can delete it or leave it, doesn't matter.
```
