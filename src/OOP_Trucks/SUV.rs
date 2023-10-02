// Second child module (SUV) with a struct that composes a parent struct and a new field
use crate::OOP_Trucks::Automotive::{Automotive, Vehicle};
pub struct SUV {
    pub car: Box<Vehicle>,
    pub cargo_capacity: Option<Box<i32>>,
}

impl Automotive for SUV {
    fn start(&self) {
        println!("{} SUV Started", self.car.name);
    }

    fn stop(&self) {
        println!("{} SUV Started", self.car.name);
    }
}
