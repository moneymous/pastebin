use mysql::*;

pub fn get_conn() -> PooledConn {
    let url = "mysql://root:@127.0.0.1:3306/pastebin";
    let pool = Pool::new(url).unwrap();
    pool.get_conn().unwrap()
}
