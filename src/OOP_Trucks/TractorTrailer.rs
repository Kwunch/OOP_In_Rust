// Third Child Module (TractorTrailer) of Truck
use crate::OOP_Trucks::{Automotive::*, Truck::Truck};
pub struct TractorTrailer {
    pub truck: Box<Truck>,
    pub trailer_length: Option<Box<f64>>,
    pub is_double: Box<bool>,
}

impl Automotive for TractorTrailer {
    fn start(&self) {
        println!("{} TractorTrailer started", self.truck.car.name);
    }

    fn stop(&self) {
        println!("{} TractorTrailer stopped", self.truck.car.name);
    }
}
