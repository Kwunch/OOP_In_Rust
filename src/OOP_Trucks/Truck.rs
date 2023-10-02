// Third child module (truck) that has its own child modules pickup truck, cement truck, tractor trailer, and fire truck
use crate::OOP_Trucks::Automotive::{Automotive, Vehicle};

#[derive(Clone)]
pub struct Truck {
    pub car: Box<Vehicle>,
    pub cargo_capacity: Option<Box<i32>>,
}

impl Automotive for Truck {
    fn start(&self) {
        println!("{} truck started", self.car.name);
    }

    fn stop(&self) {
        println!("{} truck stopped", self.car.name);
    }
}
