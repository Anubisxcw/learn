use duckdb::Connection;
use crate::config::AppConfig;

pub fn init_db(conn: Connection)  {
    conn.execute_batch(
        "CREATE SEQUENCE seq; create table if not exists alarm_cluster_info (id integer primary key default nextval('seq'),\
        cluster text not null,\
        chat_id text);"
    ).expect("create table if not exists alarm_cluster_info failed");
}