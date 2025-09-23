use chrono::{DateTime, Utc};
use mockall::automock;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub struct States {
    pub id: Option<i32>,
    pub state_id: Option<String>,
    pub started_time: Option<DateTime<Utc>>,
}

#[automock]
pub trait WorkingCycleDb {
    fn insert_state(&mut self, state_id: String, time: DateTime<Utc>);

    fn fetch_last_state(&self) -> Option<(String, DateTime<Utc>)>;
}

pub struct WorkingCycleDbSqliteImpl {
    connection: PooledConnection<SqliteConnectionManager>,
}

impl WorkingCycleDbSqliteImpl {
    pub fn new(connection: PooledConnection<SqliteConnectionManager>) -> Self {
        WorkingCycleDbSqliteImpl { connection }
    }
}

impl WorkingCycleDb for WorkingCycleDbSqliteImpl {
    fn insert_state(&mut self, state_id: String, time: DateTime<Utc>) {
        self.connection
            .execute(
                "insert into states(state_id, started_time) values (?1, ?2);",
                (state_id, time),
            )
            .unwrap();
    }

    fn fetch_last_state(&self) -> Option<(String, DateTime<Utc>)> {
        let mut stmt = self
            .connection
            .prepare("select * from states order by started_time desc limit 1;")
            .unwrap();

        let a = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>("state_id").unwrap_or_default(),
                    row.get::<_, DateTime<Utc>>("started_time")
                        .unwrap_or_default(),
                ))
            })
            .unwrap();

        let results: Vec<(String, DateTime<Utc>)> = a.into_iter().map(|s| s.unwrap()).collect();

        if results.is_empty() {
            None
        } else {
            Some(results.get(0).unwrap().to_owned())
        }
    }
}

#[cfg(test)]
pub mod common {
    use crate::db::working_cycle_db::MockWorkingCycleDb;

    pub fn get_mocked_working_cycle_trait() -> MockWorkingCycleDb {
        let mut mock = MockWorkingCycleDb::new();
        mock.expect_insert_state().returning(|_, _| ());

        mock
    }
}
