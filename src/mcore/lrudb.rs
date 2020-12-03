extern crate chrono;
extern crate rusqlite;

use self::{chrono::TimeZone, rusqlite::params};

use std::{path::Path, sync::Mutex};

pub struct LruResult {
    pub data: String,
    pub time: chrono::DateTime<chrono::Local>,
}

pub struct LruDB {
    conn: Mutex<rusqlite::Connection>,
}

type Result<T> = ::std::result::Result<T, rusqlite::Error>;

impl LruDB {
    /// Add data at scope, keep last max_n entries
    pub fn add(&self, scope: &str, s: &str, max_n: i32) -> Result<()> {
        debug!("Adding `{}` to scope `{}`", s, scope);
        let conn = self.conn.lock().unwrap();
        let now = chrono::Local::now().timestamp();
        conn.execute(
            "INSERT OR REPLACE INTO lrudata (scope, data, time) VALUES (?, ?, \
			 ?)",
            params![&scope, &s, &now],
        )?;
        conn.execute(
            "DELETE FROM lrudata WHERE scope = ? AND id NOT IN
                      (SELECT id FROM lrudata WHERE scope = ? ORDER BY time \
			 DESC, id DESC LIMIT ?)",
            params![&scope, &scope, &max_n],
        )?;
        Ok(())
    }

    /// Get all data in order
    pub fn getall(&self, scope: &str) -> Result<Vec<LruResult>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT data, time FROM lrudata WHERE scope = ? ORDER BY time \
			 DESC, id DESC",
        )?;
        let data_iter = stmt.query_map(&[&scope], |row| {
            Ok(LruResult {
                data: row.get(0)?,
                time: chrono::Local.timestamp(row.get(1)?, 0),
            })
        })?;
        let mut ret: Vec<LruResult> = Vec::new();
        for data in data_iter {
            ret.push(data.unwrap());
        }
        Ok(ret)
    }

    pub fn getall_textonly(&self, scope: &str) -> Result<Vec<String>> {
        Ok(self.getall(scope)?.into_iter().map(|x| x.data).collect())
    }

    pub fn new(dbpath: Option<&Path>) -> Result<LruDB> {
        let conn = if let Some(dbpath) = dbpath {
            rusqlite::Connection::open(dbpath)?
        } else {
            rusqlite::Connection::open_in_memory()?
        };

        conn.execute(
            "CREATE TABLE IF NOT EXISTS lrudata (
                id INTEGER PRIMARY KEY,
                scope TEXT,
                data TEXT,
                time INTEGER,
                UNIQUE (scope, data)
            )",
            params![],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS scope_time_id_idx ON lrudata (scope, \
			 time, id)",
            params![],
        )?;

        Ok(LruDB {
            conn: Mutex::new(conn),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lrudb_test() {
        let lru = LruDB::new(None).unwrap();
        lru.add("test", "hello", 3).unwrap();
        lru.add("test", "world", 3).unwrap();
        assert_eq!(lru.getall_textonly("test").unwrap(), vec!["world", "hello"]);

        lru.add("test", "hello", 3).unwrap();
        assert_eq!(lru.getall_textonly("test").unwrap(), vec!["hello", "world"]);

        lru.add("test", "1", 3).unwrap();
        lru.add("test", "2", 3).unwrap();
        assert_eq!(
            lru.getall_textonly("test").unwrap(),
            vec!["2", "1", "hello"]
        );
    }
}
