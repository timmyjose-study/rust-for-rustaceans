//! using marker types to control which methods are available when

use std::default::Default;

struct Grounded;
struct Launched;

#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
    NoColor,
}

impl Default for Color {
    fn default() -> Self {
        Color::NoColor
    }
}

struct Rocket<Stage = Grounded> {
    color: Color,
    weight: usize,
    stage: std::marker::PhantomData<Stage>, // since `Stage` is a phantom type
}

impl Rocket<Grounded> {
    pub fn launch(self) -> Rocket<Launched> {
        Rocket {
            color: self.color,
            weight: self.weight,
            stage: std::marker::PhantomData,
        }
    }
}

impl Rocket<Launched> {
    fn accelerate(&self) {
        println!("accelerating");
    }

    fn decelerate(&self) {
        println!("decelerating");
    }
}

impl Default for Rocket<Grounded> {
    fn default() -> Self {
        Rocket {
            color: Color::default(),
            weight: usize::default(),
            stage: std::marker::PhantomData,
        }
    }
}

fn main() {
    let mut rocket = Rocket::default();
    rocket.color = Color::Green;
    rocket.weight = 12345;
    println!("color = {:?}, weight = {}", rocket.color, rocket.weight);
    // this will not work since the types don't match up
    //rocket.accelerate();

    let rocket = rocket.launch();
    rocket.accelerate();
    rocket.decelerate();
    println!("color = {:?}, weight = {}", rocket.color, rocket.weight);
}
