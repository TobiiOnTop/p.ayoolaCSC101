use std::io;
 
 fn main() {
 	println!("Cafe Menu");
 	println!("Code     Item     Price");
 	println!(" T       Tea      800");
 	println!(" C      Coffee    1,200");
 	println!(" S     Sandwich   2,000");
 	println!(" J      Juice     1,500");
    

   loop {
   	
   
    println!("Please enter Item Code");
    let mut item1 = String::new();
    io::stdin().read_line(&mut item1).expect("Input is Invalid!");
    let selection1:f32= item1.trim().parse().expect("Item is Invalid!");

   println!("If you wish to quit please type '0'");
   println!("Otherwise Enter item Code");
if selection1 == 0.0 {
	break;
    }
else {println!("Please enter Item 2 Code")};
    let mut item2 = String::new();
    io::stdin().read_line(&mut item2).expect("Input is Invalid!");
    let selection2:f32= item2.trim().parse().expect("Item is Invalid!");

println!("If you wish to quit please type '0'");
   println!("Otherwise Enter item Code");
if selection2 == 0.0 {
	break;
    }
    // }







   
    	// if item1 = T 
    	// let
   







 }
}
