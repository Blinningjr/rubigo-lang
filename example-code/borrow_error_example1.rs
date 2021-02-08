let mut local: i32 = 0;
let x: &mut i32 = &mut local;
let y: &mut i32 = &mut local;
*x = 1;
*y = 2;
