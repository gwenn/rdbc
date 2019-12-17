use std::rc::Rc;
use std::cell::RefCell;

use rdbc::{Connection};
use rdbc_mysql::MySQLDriver;
use rdbc_postgres::PostgresDriver;

fn main() {

    let conn = connect_postgres();

    execute(conn, "SELECT 1");


}

//fn connect_mysql() -> Rc<RefCell<dyn Connection>> {
//    let driver = MySQLDriver::new();
//    driver
//        .connect("mysql://root:secret@127.0.0.1:3307")
//        .unwrap()
//
//}

fn connect_postgres() -> Rc<RefCell<dyn Connection>> {
    let driver = PostgresDriver::new();
    driver.connect("postgres://rdbc:secret@127.0.0.1:5433")

}

fn execute(conn: Rc<RefCell<dyn Connection>>, sql: &str) {

    let mut conn = conn.borrow_mut();
    let rs = conn.execute_query(sql).unwrap();
    let mut rs = rs.borrow_mut();

    while rs.next() {
        println!("{:?}", rs.get_i32(1))
    }

}
