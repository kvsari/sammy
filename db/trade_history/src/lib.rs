#[macro_use] extern crate derive_getters;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate chrono;
extern crate bigdecimal;
extern crate rust_decimal;

extern crate common;

mod schema;
pub mod model;
pub mod crud;
pub mod error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
