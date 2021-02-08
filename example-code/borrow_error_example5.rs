let a: i32 = 10;
let b: &mut i32 = &mut a;
let c: & i32 = &a;

if false {
    let d: i32 = 4 + *c;
} else {
    *b = 2;
}

let d: i32 = 4 + *c;
let e: i32 = *d;

