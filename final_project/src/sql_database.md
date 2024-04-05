# SQL Database

For this component, you need the `DistributionCenter` trait.
If you don't have it yet, do the [Storage Backend](storage_backend.md) guide first and then come back here.

SQL databases are ubiquitous and the ability to use them is a marketable skill.
You don't need to know SQL to follow this guide, but the provided code snippets won't be explained either.
At the start of a new project, we have to ask ourselves which database to pick, like PostgreSQL, MySQL etc.
We also have to decide if and what library we are going to use to make queries from our general-purpose programming language.

If we wanted to pull out the big guns, we could go with PostgreSQL and a full-blown ORM like [diesel](https://diesel.rs/).
But for our purposes, we'll travel a little lighter with SQLite as our database and [sqlx](https://docs.rs/sqlx/latest/sqlx/) as our query library.

## Connecting to the database

In `paekli-core`, create a new storage backend like you've probably done before:

```rust
struct SqlDatabase;

impl DistributionCenter for SqlDatabase {
    // ...
}
```

`sqlx` is database agnostic and provides [compile-time checked queries](https://github.com/launchbadge/sqlx?tab=readme-ov-file#sqlx-is-not-an-orm) without abstracting the raw power of SQL away from you.
It is also fundamentally `async`, a language feature we did not discuss.
Luckily, `async` is not necessary to understand the rest of what's going on, so it won't be explained here either.

Let's add the dependency with the feature flags `sqlite` and `runtime-tokio`, which is necessary to run `async` code.
We'll need to use the runtime `tokio` directly as well, let's add it with the full feature set.

```sh
cargo add sqlx --features sqlite,runtime-tokio
cargo add tokio --features full
```

In contrast to the file system storage or the HTTP client, the SQL database needs some initialization code.

```rust
impl SqlDatabase {
    fn new() -> Self {
        // ...
    }
}
```

To determine the location to store our database, we'll use the `directories` crate.
You've probably already done this for the CLI.

```rust
let project_dir = directories::ProjectDirs::from("dev", "buenzli", "paekli")
    .expect("the user's home directory seems to be corrupt");

let storage_dir = project_dir.data_dir();

std::fs::create_dir_all(storage_dir).expect("failed to create storage directory");

let db_path = storage_dir.join("db.sqlite");
if !db_path.exists() {
    std::fs::File::create(&db_path).expect("failed to create database");
}
let db_url = format!("sqlite:{}", db_path.display());
```

Next, we need to create a database connection pool.
The way to do that with `sqlx` is an `async` task, so we need a `tokio` runtime to execute it.
We'll also need the runtime to execute queries later, so we'll store it in a variable `rt`.

```admonish info title="realistic async" collapsible=true
This is not really how you would do `async` programming in a serious project.
It's just the simplest way to sweep the `async` stuff under the rug.
Don't worry about it, just make a mental note that for a serious `async` project, we'd do things differently.
```

```rust
let rt = tokio::runtime::Runtime::new().unwrap();
let pool_task = SqlitePool::connect(&db_url);
let pool = rt.block_on(pool_task).unwrap();
```

```admonish question title="A connection pool to a SQLite database? What?" collapsible=true
You're right, a connection pool doesn't really make sense in the context of SQLite.
However, to be database agnostic, `sqlx` uses the same abstractions for SQLite as for PostgreSQL etc.
We _could_ create a single connection to SQLite, but then we'd need a mutable reference to it to execute queries.
Connection pools in `sqlx` have the additional convenience that queries can be executed on an immutable reference to them.
```

## Initial migrations

Now that we have an open database connection, we need to create the schema.

`sqlx` has a built-in feature for migrations.
It allows you to store them as scripts in some directory and automatically execute all of them.
However, since we just need a single table, we'll keep it simple and use a regular query.

```rust
let create_table_task = sqlx::query(
    "
    CREATE TABLE IF NOT EXISTS paekli (
        content TEXT
    )
    ",
)
.execute(&pool);
rt.block_on(create_table_task).unwrap();
```

## Storing paekli

We can finally start implementing the functionality of `DistributionCenter`.
Here's the query to insert a paekli into the database.
This `async` task needs to be executed on the `tokio` runtime with `.block_on()`.

```rust
sqlx::query("INSERT INTO paekli VALUES (?)")
    .bind(content)
    .execute(&pool)
```

```admonish important title="Prepared queries"
The `?` in the query string and `.bind(content)` are executed as a _prepared statement_.
Prepared statements have built-in protection against SQL injection (a common security vulnerability).

You should **NEVER** construct a SQL query from user input with normal string manipulation like the `format!()` macro.
```

## Retrieving paekli

A reasonable approach for retrieving paekli is a query like the following.
(`rowid` is automatically added to every table by SQLite.)

```rust
let select_task = sqlx::query(
    "
    SELECT rowid, content FROM paekli
    LIMIT 1
    ",
)
.fetch_one(&pool)
```

This would work, but the returned values would be an SQL row, not the most convenient format.
Ideally, we want the result to be filled directly into a nice Rust type.
We can do that with `query_as` and a derived `FromRow` implementation on our own type:

```rust
#[derive(sqlx::FromRow)]
struct PaekliRow {
    rowid: i64,
    content: String,
}

let select_task = sqlx::query_as().fetch_one(&pool);

let PaekliRow { rowid, content } = rt.block_on(select_task).unwrap();
```

Instead of calling `.unwrap()`, we should handle the case where no paekli exist.

Lastly, execute another query to delete the retrieved paekli from the database.
The SQL query to delete a row with a specific ID is: `DELETE FROM paekli WHERE rowid = ?`.

```admonish success
The implementation of `DistributionCenter` is complete.
Now you can extend your constructor function for `DistributionCenter` and enable your clients to select the new backend.

To enable all additional features for this storage backend, you might need a little more knowledge about SQL than what we've seen so far... good luck!
```
