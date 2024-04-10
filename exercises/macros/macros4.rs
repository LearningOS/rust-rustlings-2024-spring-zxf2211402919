// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.



mod macros {
    #[macro_export]
    #[rustfmt::skip]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
        ($val:expr) => {
            println!("Look at this other macro: {}", $val);
        };
    }

    pub use my_macro; 
}

fn main() {
    macros::my_macro!();
    macros::my_macro!(7777);
}

