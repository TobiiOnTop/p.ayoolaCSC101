fn main() {
    // List of sales amounts
    let toshiba:f64 = 450000.0;
    let mac:f64 = 1500000.0;
    let hp:f64 = 750000.0;
    let dell:f64 = 2850000.0;
    let acer:f64 = 250000.0;

    // Calculate the sum
    let total = (toshiba * 2.0) + (mac) + (hp * 3.0 ) + (dell * 3.0) + acer;

    // Calculate the average
    let average = total / 10.0;

    println!("Total Sales Amount is {}", total);
    println!("Average Sales Amount is {}", average);
}
