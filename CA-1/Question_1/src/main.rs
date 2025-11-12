use std::io;

fn main() {

	println!("Enter Your Name");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Input is Invalid!");
	let name:String = input1;
	
	println!("Enter Amount of Units Consumed");
	let mut input2 = String::new();
	io::stdin().read_line(&mut input2).expect("Input is Invalid!");
	let units:f32 = input2.trim().parse().expect("Amount Of units is Invalid!");

	if units >= 0.0 && units <= 100.0 
	{
		let bill:f32 = units * 20.0;
		println!("Dear {}",name );
		println!("Your Rate per Unit is ₦20");
		println!("Your Amount of Units is {}", units);
		println!(" Your Bill is ₦{}",bill );
	}	
	else if units >= 101.0 && units <= 300.0 
	{
		let bill:f32 = units * 35.0;
		println!("Dear {}",name );
		println!("Your Rate per Unit is ₦35");
		println!("Your Amount of Units is {}", units);
		println!(" Your Bill is ₦{}",bill );
	}
	else if units >= 301.0 && units <= 500.0 
	{
		let bill:f32 = units * 50.0;
		println!("Dear {}",name );
		println!("Your Rate per Unit is ₦50");
		println!("Your Amount of Units is {}", units);
		println!(" Your Bill is ₦{}",bill );
	}
	
	else if units >= 500.0
	{
		let bill:f32 = (units * 50.0) + 5000.0;
		println!("Dear {}",name );
		println!("Your Rate per Unit is ₦50");
		println!("Your Amount of Units is {}", units);
		println!(" Your Bill is ₦{}",bill );
	}



}