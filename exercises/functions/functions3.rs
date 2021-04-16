// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

// done 

// solution: add argument to the function call
fn main() {
    call_me(9);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
