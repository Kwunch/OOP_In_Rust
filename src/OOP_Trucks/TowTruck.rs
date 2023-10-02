// Second Child Module (TowTruck) of Truck
use crate::OOP_Trucks::{Automotive::*, Truck::Truck};
pub struct TowTruck {
    pub truck: Box<Truck>,
    pub bed_size: Option<Box<f64>>,
    pub is_flatbed: Box<bool>,
}

impl Automotive for TowTruck {
    fn start(&self) {
        println!("{} TowTruck started", self.truck.car.name);
    }

    fn stop(&self) {
        println!("{} TowTruck stopped", self.truck.car.name);
    }
}
