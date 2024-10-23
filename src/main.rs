mod hannah { pub mod documentation; }
mod jerwin { pub mod refactor; }
mod leonhard { pub mod testing; pub mod numbers; pub mod palindrome; }
mod marius { pub mod code_regeneration;}
mod sanchie {pub mod debugging;}
mod advanced_code_editing{pub mod format; pub mod todo;}

fn main() {
    hannah::documentation::documentation();
    jerwin::refactor::refactor();
    leonhard::testing::testing();
    marius::code_regeneration::code_generation();
    sanchie::debugging::debugging();
    advanced_code_editing::format::cartesian_product("stephen", "bscs3");
}


