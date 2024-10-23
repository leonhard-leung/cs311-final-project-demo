use crate::documentation::documentation::Documentation;

mod documentation { pub mod documentation; }
mod refactoring { pub mod refactor; }
mod testing { pub mod testing; pub mod numbers; pub mod palindrome; }
mod debugging {pub mod debugging;}
mod advanced_code_editing{pub mod format; pub mod todo;}

mod code_generation {
    pub mod code_regeneration;
    pub mod code_gen_derive;
    pub mod code_gen_create_structs;
    pub mod code_gen_traits_and_structs;
    pub mod code_gen_simple_macro;
}

fn main() {
    Documentation::add(10,10);
    refactoring::refactor::refactor();
    testing::testing::testing();
    debugging::debugging::debugging();
    advanced_code_editing::format::cartesian_product("stephen", "bscs3");

    code_generation::code_regeneration::code_generation();
    // code_generation::code_gen_derive::main();
    // code_generation::code_gen_create_structs::main();
    // code_generation::code_gen_traits_and_structs::main();
    // code_generation::code_gen_simple_macro::main();
}


