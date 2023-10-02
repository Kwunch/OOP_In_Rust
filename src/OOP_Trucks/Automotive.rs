// Parent module containing a trait and a struct
pub trait Automotive {
    fn start(&self);
    fn stop(&self);
}

#[derive(Clone)]
pub struct Vehicle {
    pub name: String,
    pub year: i32,
}

impl Automotive for Vehicle {
    fn start(&self) {
        println!("{} started", self.name);
    }

    fn stop(&self) {
        println!("{} stopped", self.name);
    }
}
