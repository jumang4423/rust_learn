use std::thread;
use std::time::Duration;
use rand::{thread_rng, Rng};

struct Ryoma {
    name: String,
    age: i32,
}

trait His { 
    fn say_hi(&self); }

impl His for Ryoma {
    fn say_hi(&self) {
        fn run_pow(x: i32) -> i32 {
            return x * x;
        }
        fn he18_age(age: i32) -> Option<i32> {
            if age == 18 {
                Some(18)
            } else {
                None
            }
        }

        println!("hi, i am {} ", self.name);

        match he18_age(self.age){
            Some(age) => println!("and i am {} years old.", age),
            None => println!("and i am not 18 years old."),
        }
        println!("5 x 5 equals {}", run_pow(5));
    }
}




fn main() {
    let ryoma: Ryoma = Ryoma {
        name: String::from("ryoma"),
        age: 14,
    };

    let hi10: std::thread::JoinHandle<()> = thread::spawn(|| {

        let mut rng = rand::thread_rng();
        for _i in 1..10 {
            println!("hi, random number is {}", rng.gen_range(0, 10000));
            thread::sleep(Duration::from_millis(100));
        }

    });
    ryoma.say_hi();
    hi10.join().unwrap();
}
