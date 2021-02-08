let a: i32 = 3;
let mut b: & i32 = &a;
{
    let c: i32 = 4;
    b = &c;
}
print(*b);

