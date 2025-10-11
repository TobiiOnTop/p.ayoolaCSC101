fn main() {
    // Given values
    let principal:f64 = 510000.0;  
    let rate:f64 = 5.0;            
    let years:f64 = 3.0;           

    // Calculate depreciation using formula: A = P * (1 - R/100)^n
    let amount = principal * (1.0 - rate / 100.0).powf(years);


    println!("Value of TV is {}", amount);
}


