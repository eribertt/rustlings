// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

// DONE

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
	//remove the semicolon to make it an expression
    num * num
}
