mod OOP_Trucks;

use std::{io, io::Write, thread, time};
use OOP_Trucks::*;

fn main() {
    let vechicle: Box<Automotive::Vehicle> = Box::new(Automotive::Vehicle {
        name: "Vehicle".to_string(),
        year: 1932,
    });
    let sports_car: Box<SportsCar::SportsCar> = Box::new(SportsCar::SportsCar {
        car: vechicle.clone(),
        top_speed: Box::new(180.0),
    });
    let suv: Box<SUV::SUV> = Box::new(SUV::SUV {
        car: vechicle.clone(),
        cargo_capacity: Some(Box::new(9)),
    });
    let truck: Box<Truck::Truck> = Box::new(Truck::Truck {
        car: vechicle.clone(),
        cargo_capacity: Some(Box::new(8)),
    });

    // Child Truck Modules
    let pickup_truck: Box<PickupTruck::PickupTruck> = Box::new(PickupTruck::PickupTruck {
        truck: truck.clone(),
        bed_size: Some(Box::new(12.5)),
        is_4wd: Box::new(false),
    });
    let tow_truck: Box<TowTruck::TowTruck> = Box::new(TowTruck::TowTruck {
        truck: truck.clone(),
        bed_size: Some(Box::new(48.0)),
        is_flatbed: Box::new(false),
    });
    let tractor_trailer: Box<TractorTrailer::TractorTrailer> =
        Box::new(TractorTrailer::TractorTrailer {
            truck: truck.clone(),
            trailer_length: Some(Box::new(138.5)),
            is_double: Box::new(true),
        });

    static mut vechicle_vec: Vec<Box<dyn Automotive::Automotive>> = Vec::new();

    unsafe {
        vechicle_vec.push(vechicle);
        vechicle_vec.push(sports_car);
        vechicle_vec.push(suv);
        vechicle_vec.push(truck);

        vechicle_vec.push(pickup_truck);
        vechicle_vec.push(tow_truck);
        vechicle_vec.push(tractor_trailer);

        let fleet: Box<Fleet::Fleet> = Box::new(Fleet::Fleet {
            cars: &vechicle_vec,
            company_name: Some(Box::new("Software Engineering Corp".to_string())),
        });

        vechicle_vec.push(fleet);
        let two_seconds = time::Duration::from_millis(500);
        loop {
            println!(
                "
        1. Vehicle
        2. Sports Car
        3. SUV
        4. Truck
        5. Pickup Truck
        6. Tow Truck
        7. Tractor Trailer
        8. Fleet
        9. All
        "
            );

            print!(
                "Enter Vehicle [1-9]
        : "
            );
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            let input: i8 = input.trim().parse().expect("Invalid Input");

            if input == 9 {
                print_all(&vechicle_vec);
                continue;
            } else {
                let temp_vehicle = vechicle_vec.get((input - 1) as usize).unwrap();

                temp_vehicle.start();
                thread::sleep(two_seconds);
                temp_vehicle.stop();
            }

        }
    }
}

fn print_all(vehicle: &Vec<Box<dyn Automotive::Automotive>>) {
    let two_seconds = time::Duration::from_millis(500);
    for automotive in vehicle {
        thread::sleep(two_seconds);
        automotive.start();
        thread::sleep(two_seconds);
        automotive.stop();
    }
}
