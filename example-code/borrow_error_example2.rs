fn example(x: &mut i32, y: &mut i32) -> mut i32 {
    *x = 42;
    *y = 14;
    return *x;
}

let mut local: i32 = 5;
let pointer: &mut i32 = &mut local;
let result: i32 = example(pointer, pointer);

