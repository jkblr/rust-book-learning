use std::io;

fn main() {
    loop {
        println!("Input the unit you want to convert! (Either C or F)");

        let mut unit = String::new();

        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");
        println!("Input the temperature value you wish to convert!");

        let temp: f32 = match unit.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let res = F32::new();
        let new_unit = String::new();
        if unit == "F" {
            let res = 5 / 9 * (temp - 32.0);
            let new_unit = "C";
        } else if unit == "C" {
            let res = 9 / 5 * temp + 32;
            let new_unit = "F";
        }
        println!("Your temperature converts to {res} {new_unit}");
    }
}
