// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

// Solved!

fn main() {
    call_me(3);
}

//declare num of type signed integer
fn call_me(num: i64) {
	let num = 9;
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
