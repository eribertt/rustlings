// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

// finish may 14, 2021 DONE

fn main() {
    //let a = ???
	// a is an array of 10 elements of 150 value each
    let a = [10; 150];
	//this is error; println! cannot print array
	//println!("The value of a is {}", a);
	//formatting is a separate topic all together
	//https://doc.rust-lang.org/stable/rust-by-example/hello/print.html

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
