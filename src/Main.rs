use crate::documentation::documentation::Documentation;

mod documentation { pub mod documentation; }
mod refactoring { pub mod refactor; }
mod testing { pub mod numbers; pub mod palindrome; }
mod debugging {pub mod debugging;}
mod advanced_code_editing{pub mod format; pub mod todo;}

mod code_generation
    {
     pub mod derive;
     pub mod create_structs;
     pub mod traits_and_structs;
     pub mod simple_macro;
    }

fn main()
    {
     Documentation::add(10,10);
     refactoring::refactor::refactor();
     debugging::debugging::debugging();
     advanced_code_editing::format::cartesian_product("stephen", "bscs3");

     code_generation::derive::main();
     // code_generation::simple_macro::main();
    }


