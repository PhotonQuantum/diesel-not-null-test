use diesel::{AsExpression, Connection, FromSqlRow, Insertable, Queryable, RunQueryDsl, serialize, SqliteConnection};
use diesel::backend::RawValue;
use diesel::deserialize::FromSql;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;

use crate::schema::kvs;

mod schema;

#[derive(Queryable, Insertable)]
#[diesel(table_name = kvs)]
struct KV {
    pub k: i32,
    pub v: CustomWrapper,
}

fn main() {
    let mut db = SqliteConnection::establish("db.sqlite").unwrap();
    diesel::insert_into(kvs::table)
        .values(&KV { k: 4, v: CustomWrapper("".to_string()) })
        .execute(&mut db)
        .unwrap();
}

#[derive(Debug, FromSqlRow, AsExpression)]
#[diesel(sql_type = Text)]
struct CustomWrapper(String);

impl FromSql<Text, Sqlite> for CustomWrapper {
    fn from_sql(bytes: RawValue<'_, Sqlite>) -> diesel::deserialize::Result<Self> {
        let s = FromSql::<Text, Sqlite>::from_sql(bytes)?;
        Ok(CustomWrapper(s))
    }
}

impl ToSql<Text, Sqlite> for CustomWrapper {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.0.to_string());
        Ok(serialize::IsNull::No)
    }
}