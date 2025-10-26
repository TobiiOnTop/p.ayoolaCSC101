use std::io;

fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

	println!("Program to find the Roots of a Quadratic Equation");

	println!("Enter a");
	io::stdin().read_line(&mut input1).expect("Not a valid string");
	let a:i32 = input1.trim().parse().expect("Not a valid number");

	println!("Enter b");
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let b:i32 = input2.trim().parse().expect("Not a valid number");

	println!("Enter c");
	io::stdin().read_line(&mut input3).expect("Not a valid string");
	let c:i32 = input3.trim().parse().expect("Not a valid number");

	let determinant:i32  = (b * b) - (4 * a *c);
	println!("The Determinant of the Quadratic Equation is {}", determinant);

	if determinant > 0 {
		println!("There are two Distinct Roots.");
	}

	else if determinant == 0 {
		println!("There is exactly one Real Root.");
	}

	else if determinant < 0 {
		println!("There are no Real Roots.");
	}
    if determinant >= 0 {

    let sqrt_d = determinant.isqrt(); 
        let root1 = (-b + sqrt_d) / (2 * a);
        let root2 = (-b - sqrt_d) / (2 * a);

        println!("Root 1 = {}", root1);
        println!("Root 2 = {}", root2);
    }
     else {
    println!("There are no real roots (complex roots).")
}

}