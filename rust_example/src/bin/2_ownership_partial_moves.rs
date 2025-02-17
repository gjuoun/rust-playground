#[derive(Debug)]
struct Device {
    brand: String,
    year: u16,
}

fn main() {
    let device = Device {
        brand: String::from("Acme"),
        year: 2021,
    };

    // Move `brand`, reference `year`
    let Device { brand, ref year } = device;

    println!("Device ref year: {}", year);
    println!("Device direct access year: {}", device.year);
    println!("Device brand: {}", brand);

    // Error! brand is moved out from device
    // println!("Device brand: {}", device.brand);

    // Error! Cannot use `device` after partial move
    // println!("Device: {:?}", device);

    // `device.year` can still be used
    println!("Device year from struct: {}", device.year);
}
