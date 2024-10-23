use crate::documentation::documentation::Documentation;

mod documentation { pub mod documentation; }
mod refactoring { pub mod refactor; }
mod testing { pub mod testing; pub mod numbers; pub mod palindrome; }
mod marius { pub mod code_regeneration;}
mod debugging {pub mod debugging;}
mod advanced_code_editing{pub mod format; pub mod todo;}

fn main() {
    Documentation::add(10,10);
    refactoring::refactor::refactor();
    testing::testing::testing();
    marius::code_regeneration::code_generation();
    debugging::debugging::debugging();
    advanced_code_editing::format::cartesian_product("stephen", "bscs3");
}