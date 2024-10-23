mod hannah { pub mod documentation; }
mod jerwin { pub mod refactor; }
mod leonhard { pub mod testing; }
mod code_generation {
    pub mod code_regeneration;
    pub mod code_gen_derive;
    pub mod code_gen_create_structs;
    pub mod code_gen_traits_and_structs;
    pub mod code_gen_simple_macro;
}
mod sanchie {pub mod debugging;}
mod stephen { pub mod code_editing; }

fn main() {
    hannah::documentation::documentation();
    jerwin::refactor::refactor();
    leonhard::testing::testing();
    code_generation::code_regeneration::code_generation();
    // code_generation::code_gen_derive::main();
    // code_generation::code_gen_create_structs::main();
    // code_generation::code_gen_traits_and_structs::main();
    // code_generation::code_gen_simple_macro::main();
    sanchie::debugging::debugging();
    stephen::code_editing::code_editing();
}


