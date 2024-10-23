trait Vehicle {
    fn mileage(&self) -> u32;
    fn fuel_type(&self) -> String;
}

struct Car {
    name: String,
    manufacturer: String,
}

struct Airplane {
    manufacturer: String,
    plane_type: String,
}

//todo: implement Vehicle for Car/Airplane












// pub fn main() {
//     let car = Car {
//         name: "Model S".to_string(),
//         manufacturer: "Tesla".to_string(),
//     };
//
//     let airplane = Airplane {
//         manufacturer: "Boeing".to_string(),
//         plane_type: "747".to_string(),
//     };
//
//     println!("Car: {}, Mileage: {}, Fuel Type: {}", car.name, car.mileage(), car.fuel_type());
//     println!("Airplane: {}, Mileage: {}, Fuel Type: {}", airplane.plane_type, airplane.mileage(), airplane.fuel_type());
// }
