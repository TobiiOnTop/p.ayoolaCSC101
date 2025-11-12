use std::io;

fn main() {
   loop {
  let mut input1 = String::new();
  let mut input2 = String::new();
  let mut input3 = String::new();
  let mut input4 = String::new();
  


 	
  println!("Enter the Principal Amount");
	io::stdin().read_line(&mut input1).expect("Input is Invalid!");
	let P:f32 = input1.trim().parse().expect("Principal is Invalid!");

 println!("Enter the Rate");
	io::stdin().read_line(&mut input2).expect("Input is Invalid!");
	let R:f32 = input2.trim().parse().expect("Rate is Invalid!");

  println!("Enter the Time ");
	io::stdin().read_line(&mut input3).expect("Input is Invalid!");
	let T:f32 = input3.trim().parse().expect("Time is Invalid!");

	let A:f32  = P * (1.0 + (R / 100.0) ) ;

	let I:f32 = A - P;
	println!("Amount Owed is ₦{}",A );
	println!("Local Interest is ₦{}",I );

	 println!("Do you wish to Calculate for another User");
	 println!("Enter yes/no as 1/2");
     io::stdin().read_line(&mut input4).expect("Input is Invalid!");
     let value:f32 = input4.trim().parse().expect("Amount Of units is Invalid!");
     if value == 2.0
     { 
      break;		
       	}  	
       }  
	

}
