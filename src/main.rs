mod hannah { pub mod documentation; }
mod jerwin { pub mod refactor; }
mod leonhard { pub mod testing; pub mod numbers; pub mod palindrome; }
mod marius { pub mod code_regeneration;}
mod sanchie {pub mod debugging;}
mod stephen { pub mod code_editing; }

fn main() {
    hannah::documentation::documentation();
    jerwin::refactor::refactor();
    leonhard::testing::testing();
    marius::code_regeneration::code_generation();
    sanchie::debugging::debugging();
    stephen::code_editing::code_editing();
}


