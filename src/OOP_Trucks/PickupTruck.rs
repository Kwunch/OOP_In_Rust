// First Child Module (PickupTruck) of Truck
use crate::OOP_Trucks::{Automotive::*, Truck::Truck};
pub struct PickupTruck {
    pub truck: Box<Truck>,
    pub bed_size: Option<Box<f64>>,
    pub is_4wd: Box<bool>,
}

    impl Automotive for PickupTruck {
    fn start(&self) {
        println!("{} pickup truck started", self.truck.car.name);
    }

        fn stop(&self) {
        println!("{} pickup truck stopped", self.truck.car.name);
    }
}