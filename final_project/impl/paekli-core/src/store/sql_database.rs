use sqlx::SqlitePool;

use super::DistributionCenter;

pub struct SqlDatabase {
    rt: tokio::runtime::Runtime,
    pool: SqlitePool,
}

impl SqlDatabase {
    pub fn new() -> Self {
        let project_dir = directories::ProjectDirs::from("dev", "buenzli", "paekli")
            .expect("the user's home directory seems to be corrupt");
        let storage_dir = project_dir.data_dir();
        std::fs::create_dir_all(storage_dir).expect("failed to create storage directory");
        let db_path = storage_dir.join("db.sqlite");
        if !db_path.exists() {
            std::fs::File::create(&db_path).unwrap();
        }
        let db_url = format!("sqlite:{}", db_path.display());

        let rt = tokio::runtime::Runtime::new().unwrap();
        let pool_task = SqlitePool::connect(&db_url);
        let pool = rt.block_on(pool_task).unwrap();

        let create_table_task = sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS paekli (
                content TEXT,
                -- optional:
                receiver TEXT,
                express BOOLEAN
            )
            ",
        )
        .execute(&pool);
        rt.block_on(create_table_task).unwrap();

        Self { rt, pool }
    }
}

impl DistributionCenter for SqlDatabase {
    fn store(&self, receiver: String, content: String, express: bool) {
        let insert_task = sqlx::query("INSERT INTO paekli VALUES (?, ?, ?)")
            .bind(content)
            .bind(receiver)
            .bind(express)
            .execute(&self.pool);
        self.rt.block_on(insert_task).unwrap();
    }

    fn retrieve(&self, receiver: String) -> Option<String> {
        #[derive(sqlx::FromRow)]
        struct PaekliRow {
            rowid: i64,
            content: String,
        }

        let select_task = sqlx::query_as(
            "
            SELECT rowid, content FROM paekli
            WHERE receiver = ?
            LIMIT 1
            ",
        )
        .bind(receiver)
        .fetch_one(&self.pool);
        let PaekliRow { rowid, content } = self.rt.block_on(select_task).ok()?;

        let insert_task = sqlx::query("DELETE FROM paekli WHERE rowid = ?")
            .bind(rowid)
            .execute(&self.pool);
        self.rt.block_on(insert_task).unwrap();

        Some(content)
    }
}

#[test]
fn it_works() {
    let db = SqlDatabase::new();
    db.store("sqlite_unit_test".into(), "whatever".into(), false);
    assert_eq!(
        db.retrieve("sqlite_unit_test".into()),
        Some("whatever".into())
    );
    assert_eq!(db.retrieve("sqlite_unit_test".into()), None);
}
