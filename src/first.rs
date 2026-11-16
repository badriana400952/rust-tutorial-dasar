
use crate::third::say_hello as say_hello_third;

pub fn say_hello() {
    println!("helloo badriana");
    say_hello_third()
}
//memanggil modul yang ada di atas nya
mod secon {
    mod third {
        pub fn panggil_hello() {
            super::super::say_hello();
        }
    }
}