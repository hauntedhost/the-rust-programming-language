fn main() {
    // tuples
    let tup: (i32, f64, u8) = (500, 3.77, 100);
    let (_x, y, _z) = tup;
    println!("the value of 'y' is {}", y);

    // arrays
    let months = [
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "august",
        "september",
        "october",
        "november",
        "december",
    ];
    println!("the second month of the year is {}", months[1]);
}
