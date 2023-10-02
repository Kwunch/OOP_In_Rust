// First child module SportsCar with a struct that contains an instance of the parent struct
use crate::OOP_Trucks::Automotive::{Automotive, Vehicle};
pub struct SportsCar {
    pub car: Box<Vehicle>,
    pub top_speed: Box<f64>,
}

impl Automotive for SportsCar {
    fn start(&self) {
        println!("{} Sports Car Started", self.car.name);
    }

    fn stop(&self) {
        println!("{} Sports Car Started", self.car.name);
    }
}
