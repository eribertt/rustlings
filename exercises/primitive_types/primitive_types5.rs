// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

// done May 14, 2021

fn main() {
    let cat = ("Furry McFurson", 3.5);
    //let /* your pattern here */ = cat;
	//this is about tuple destructuring
	//must be enclosed in parens
    let (name, age) = cat;
    println!("{} is {} years old.", name, age);
}
