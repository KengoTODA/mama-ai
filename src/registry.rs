extern crate postgres;

use std;
use std::env;
use std::ffi::OsString;
use std::io;
use value::Aqi;

pub struct AqiRegistry {
    conn: postgres::Connection,
}

impl AqiRegistry {
    fn create_table(&self) -> io::Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS aqi (
                    id    SERIAL PRIMARY KEY,
                    value SMALLINT NOT NULL,
                    time  timestamp NOT NULL DEFAULT now()
                  )",
            &[],
        )?;
        Ok(())
    }

    pub fn insert(&self, aqi: i16) -> io::Result<()> {
        self.conn
            .execute("INSERT INTO aqi (value) VALUES ($1)", &[&aqi])?;
        Ok(())
    }

    fn prune(&self) -> io::Result<u64> {
        let rows = self.conn.query("SELECT MAX(id) FROM aqi", &[])?;
        let max_id: i32 = rows.get(0).get(0);
        let deleted: u64 = self.conn
            .execute("DELETE FROM aqi WHERE id < $1", &[&max_id])?;
        Ok(deleted)
    }

    fn truncate(&self) -> io::Result<()> {
        self.conn.execute("TRUNCATE TABLE aqi", &[])?;
        Ok(())
    }

    pub fn select(&self) -> io::Result<Option<Aqi>> {
        for row in self.conn
            .query("SELECT value, time FROM aqi ORDER BY id DESC LIMIT 1", &[])?
            .iter()
        {
            let value: i16 = row.get(0);
            let time: ::chrono::NaiveDateTime = row.get(1);
            return Ok(Some(Aqi { value, time }));
        }
        Ok(None)
    }
}

fn connect_db() -> std::option::Option<postgres::Connection> {
    let url =
        env::var_os("DATABASE_URL").unwrap_or(OsString::from("postgres://postgres@localhost:5432"));
    postgres::Connection::connect(url.into_string().unwrap(), postgres::TlsMode::None).ok()
}

pub fn connect() -> AqiRegistry {
    AqiRegistry {
        conn: connect_db().unwrap(),
    }
}

// To test, run following command in advance
// docker run -it -p 5432:5432 postgres:10
#[test]
fn test_insert() {
    let conn = connect_db().expect("connection should be established");
    let dao = AqiRegistry { conn };
    dao.create_table().unwrap();
    dao.insert(100).unwrap();
    let aqi = dao.select().unwrap();
    assert_eq!(aqi.expect("AQI should be found").value, 100);
}

// to run test cases in serial, run cargo like below:
// cargo test -- --test-threads=1
#[test]
fn test_prune() {
    let conn = connect_db().expect("connection should be established");
    let dao = AqiRegistry { conn };
    dao.create_table().unwrap();
    dao.truncate().unwrap();
    dao.insert(100).unwrap();
    assert_eq!(dao.prune().unwrap(), 0);
    dao.insert(200).unwrap();
    assert_eq!(dao.prune().unwrap(), 1);
    dao.insert(300).unwrap();
    dao.insert(400).unwrap();
    assert_eq!(dao.prune().unwrap(), 2);
}
