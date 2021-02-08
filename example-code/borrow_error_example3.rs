let mut local: i32 = 42;
let x: &mut i32 = &mut local;
let shared1: &i32 = &local;
let shared2: &i32 = &local;
let mut val: i32 = *x;
val = *shared1;
val = *shared2;
*x = 17 + *x;
val = *shared1;

