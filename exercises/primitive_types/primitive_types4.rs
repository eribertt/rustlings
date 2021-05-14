// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

// done may 14, 2021

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    //let nice_slice = ???
	//use ampersand before a to borrow from array
	//array elements from 1 to 3 excluding 4th element
    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
