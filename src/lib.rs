pub mod models;
pub mod solver;
pub mod matching;
pub mod routing;
pub mod pricing;
pub mod utils;
pub mod adapters;
pub mod bridge;
pub mod daemon;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Basic sanity test to ensure the library compiles and modules are accessible.
        assert_eq!(2 + 2, 4);
    }
}
