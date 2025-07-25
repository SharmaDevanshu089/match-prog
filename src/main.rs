fn main() {
    let number_of_day = 4;
    match number_of_day {
        1 => print!("Monday"),
        2 => print!("Tuesday"),
        3 => print!("Wednesday"),
        4 => print!("Thursday"),
        5 => print!("Friday"),
        6 => print!("Saturday"),
        7 => print!("Sunday"),
        _ => print!("Invalid Date"),
    }
}
