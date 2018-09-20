#[macro_use] extern crate derive_getters;
#[macro_use] extern crate schemamama;
#[macro_use] extern crate serde_derive;
extern crate postgres;
extern crate schemamama_postgres;
extern crate serde;
extern crate chrono;
extern crate rust_decimal;

extern crate common;

//mod schema;
pub mod model;
pub mod crud;
//pub mod crud2;
pub mod migration;
pub mod error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
