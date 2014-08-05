extern crate r2d2;

use std::cell::Cell;
use std::default::Default;

#[deriving(Show, PartialEq)]
struct FakeConnection;

#[deriving(Default)]
struct OkManager;

impl r2d2::PoolManager<FakeConnection, ()> for OkManager {
    fn connect(&self) -> Result<FakeConnection, ()> {
        Ok(FakeConnection)
    }
}

struct NthConnectFailManager {
    n: Cell<uint>
}

impl r2d2::PoolManager<FakeConnection, ()> for NthConnectFailManager {
    fn connect(&self) -> Result<FakeConnection, ()> {
        let n = self.n.get();
        if n > 0 {
            self.n.set(n - 1);
            Ok(FakeConnection)
        } else {
            Err(())
        }
    }
}

#[test]
fn test_initial_size_ok() {
    let config = r2d2::Config {
        initial_size: 5,
        ..Default::default()
    };
    let manager = NthConnectFailManager { n: Cell::new(5) };
    assert!(r2d2::Pool::with_manager(config, manager).is_ok());
}

#[test]
fn test_initial_size_err() {
    let config = r2d2::Config {
        initial_size: 5,
        ..Default::default()
    };
    let manager = NthConnectFailManager { n: Cell::new(4) };
    assert_eq!(r2d2::Pool::with_manager(config, manager).err().unwrap(),
               r2d2::ConnectionError(()));
}

#[test]
fn test_acquire_release() {
    let config = r2d2::Config {
        initial_size: 2,
        ..Default::default()
    };
    let pool = r2d2::Pool::with_manager(config, OkManager).unwrap();

    let conn1 = pool.get().unwrap();
    let _conn2 = pool.get().unwrap();
    drop(conn1);
    let _conn3 = pool.get().unwrap();
}