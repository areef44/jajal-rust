use crate::third::say_hello as say_hello_third;

pub fn say_hello(){
    println!("Hello From First Module");

    say_hello_third();
}

pub mod second {
    pub mod third {
        pub fn say_hello(){
            // crate::first::say_hello();
            super::super::say_hello();
        }
    }
}
