// Fourth child module (Fleet) with a struct that composes a parent struct and a vector of child structs
use crate::OOP_Trucks::Automotive::{Automotive, Vehicle};

pub struct Fleet {
    pub cars: &'static Vec<Box<dyn Automotive>>,
    pub company_name: Option<Box<String>>,
}

impl Automotive for Fleet {
    fn start(&self) {
        println!("{:?} fleet is starting up", self.company_name);
        for i in 0..self.cars.len() - 1{
            if i == self.cars.len() {
                break;
            }
            self.cars.get(i).unwrap().start();
        }
    }

    fn stop(&self) {
        println!("{:?} fleet is shutting down", self.company_name);
        for i in 0..self.cars.len() - 1 {
            if i == self.cars.len() {
                break;
            }
            self.cars.get(i).unwrap().stop();
        }
    }
}
