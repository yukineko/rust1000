use rand::prelude::*;
struct Sheep {
    age: u32,
}

struct Cow {
    age: u32,
}

trait Animal {
    fn get_age(&self) -> u32;
}

impl Animal for Sheep {
    fn get_age(&self) -> u32 {
        self.age
    }
}

impl Animal for Cow {
    fn get_age(&self) -> u32 {
        self.age * 2
    }
}

fn random_animal(r: bool) -> Box<dyn Animal> {
    if r  {
        Box::new(Sheep {
            age: 5,
        })
    } else {
        Box::new(Cow {
            age: 10,
        })
    }
}

fn main()->Result<(), std::io::Error> {
    let mut rng = rand::rng();
    let r = rng.random_bool(1.0 / 3.0);
    let animal = random_animal(r);
    println!("Animal age: {}", animal.get_age());
    Ok(())
}