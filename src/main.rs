   fn main() {
    //Integers
    let x: i32 = -42;//both positive and negative numbers
    let y: u64 = 100;//only positive numbers
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
    let e: i32 = 2147483647;//defines the range. Going higher than the number will print an error.
    // use an alternative like u32 for unsigned since it is slightly higher than i32
    let i: i64 = 9223372036854775807;//defines the range. Going higher than the number will print an error.
    // use an alternative like u64 for unsigned since it is slightly higher than i64
    println!("Maximum value of i32: {}", e);
    println!("Maximum value of i64: {}", i);

    //Floats

    let pi: f32 = 3.14;
    println!("Value of pi: {}", pi);//prints a float value

    //Booleans

    let is_snowing: bool = true;//prints a boolean value
    println!("Is it snowing?: {}", is_snowing);

    //Characters

    let letter: char = 'a';
    println!("First letter of the alphabet: {}", letter);//prints a character value
     
     //Compound Data Types 
     //Arrays
      let numbers: [i32; 5] = [1, 2, 3, 4, 5];
      println!("Number Array: {:?}", numbers);
      //let mix = [1, 2, "apple", true];should not work since it doest contain elements of the same data types
      //println!("Mixed Array: {:?}", mix);
      let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];//use of string slices
      println!("Fruits Array: {:?}", fruits);
      println!("Fruits Array 1st element: {}", fruits [0]);//Printing of each array element
      println!("Fruits Array 2nd element: {}", fruits [1]);//Printing of each array element
      println!("Fruits Array 3rd element: {}", fruits [2]);//Printing of each array element

      //Tuples
      let human: (String, i32, bool) = ("Alice".to_string(), 30, false);//the sring slice "Alice is converted to a string using the function ("text"to_string")
      println!("Human Tuple: {:?}", human);
      let my_mix_tuple = ("Kratos", 23, true, [1,2,3,4,5]);//mix of data types in this case having a string, integer, boolean and an array 
      println!("My Mix Tuple: {:?}", my_mix_tuple);

      //Slices
      let number_slices:&[i32] = &[ 1,2,3,4,5];
      println!("Number Slice: {:?}", number_slices);

      let animal_slices :&[&str] = &["Lion", "Elephant", "Crocodile"];//defines a string slice 
      println!("Animal Slice: {:?}", animal_slices);

      let book_slices :&[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"Zen".to_string()];//defines a string
      println!("Book Slice: {:?}", book_slices);

      //Strings vs String Slices (&str)
      //Strings
      //strings are growable, expandable, increasable and decreasable hence are mutable and are owned meaning they are not borrowed 
      let mut stone_cold: String = String::from("Hell, ");
      stone_cold.push_str("yeah!");
      println!("Stone Cold Says: {}", stone_cold);

      //&str (String Slices)
      //Reference to a string meaning it's not owned but borrowed and immutable hence can't be changed
      //the Stack is fast while the heap is slower
      //the stack can't have immutable datatypes
      //the heap can have dynamic mutable datatypes
      let string: String = String::from("Hello, World!");
      let slice: &str = &string;//refers to the value of the string which is "Hello, World!"
      println!("String Value: {}", slice);

      //Functions
      tell_height(182);
      human_id("Joel", 55, 182.2);
      

}
 
 //inserting input values   
 fn tell_height(height: u32){
    println!("My Height is {} cm.", height);
 }

 //you can insert more than one value
 fn human_id(name: &str, age: u32, height: f32){
    println!("Name: {}, Age: {}, Height: {}", name, age, height);
 }