use std::io;
fn main() {
    println!("Hello");
    println!("This is a program to Calculate Area or Volume of Different Shapes");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");
    println!("Please Select an option from 1-5");

    let mut input1 = String::new();
	println!("Enter The Program you wish to run");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let selection:u8 = input1.trim().parse().expect("Invalid Input");

	if selection == 1 {
		trapezium()
	}
	else if selection == 2 {
		rhombus()
	}
	else if selection == 3 {
		parallelogram()
	}
	else if selection == 4 {
		cube()
	}
	else if selection == 3 {
		cylinder()
	}
	else {
		println!("Please Select a valid input");
	}
}

fn trapezium() {
	println!("Program to find the Area of a Trapezium");

	let mut input2 = String::new();
	println!("Enter Base1:");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let a:f32 = input2.trim().parse().expect("Invalid Input");

	let mut input3 = String::new();
	println!("Enter Base2:");
	io::stdin().read_line(&mut input3).expect("Failed to read input");
	let b:f32 = input3.trim().parse().expect("Invalid Input");

	let mut input4 = String::new();
	println!("Enter Height:");
	io::stdin().read_line(&mut input4).expect("Failed to read input");
	let h:f32 = input4.trim().parse().expect("Invalid Input");

	let area = 0.5 * (a + b) * h;

	println!("The area of this Trapezium is {} ",area);

}

fn rhombus(){
println!("Program to find the Area of a Rhombus");

	let mut input2 = String::new();
	println!("Enter Diagonal1:");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let d1:f32 = input2.trim().parse().expect("Invalid Input");

	let mut input3 = String::new();
	println!("Enter Diagonal2:");
	io::stdin().read_line(&mut input3).expect("Failed to read input");
	let d2:f32 = input3.trim().parse().expect("Invalid Input");

	

	let area = 0.5 * d1 * d2 ;

	println!("The area of this rhombus is {} ",area);

}

fn parallelogram(){
		println!("Program to find the Area of a Parallelogram");

	let mut input2 = String::new();
	println!("Enter Base:");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let b:f32 = input2.trim().parse().expect("Invalid Input");

	let mut input4 = String::new();
	println!("Enter Height:");
	io::stdin().read_line(&mut input4).expect("Failed to read input");
	let h:f32 = input4.trim().parse().expect("Invalid Input");

	let area = b * h;

	println!("The area of this Parallelogram is {} ",area);
}
fn cube() {
	println!("Program to find the Area of a Trapezium");

	let mut input4 = String::new();
	println!("Enter Length:");
	io::stdin().read_line(&mut input4).expect("Failed to read input");
	let l:f32 = input4.trim().parse().expect("Invalid Input");

	let area = 6.0 * l * l;

	println!("The area of this cube is {} ",area);

}
fn cylinder() {
	println!("Program to find the Volume of a Cylinder");


	let mut input3 = String::new();
	println!("Enter Radius:");
	io::stdin().read_line(&mut input3).expect("Failed to read input");
	let r:f32 = input3.trim().parse().expect("Invalid Input");

	let mut input4 = String::new();
	println!("Enter Height:");
	io::stdin().read_line(&mut input4).expect("Failed to read input");
	let h:f32 = input4.trim().parse().expect("Invalid Input");

	let volume = 22.0 / 7.0 * (r * r) * h;

	println!("The Volume of this Cylinder is {} ",volume);

}