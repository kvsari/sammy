//! Common models for the entire project.
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate rust_decimal;
extern crate chrono;

pub mod trade;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
