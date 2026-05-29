// #[macro_use]
mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

// use crate::my_macro;
fn main() {
    my_macro!();
}
