let mut a: i32 = 10;
let b: &mut i32 = & mut a;
*b = 2;

print(*b);
print(a);

let mut c: i32 = 20;

fn test(input: &mut i32, mut tal: i32) -> () {
	*input = *input / 2;
	tal = 2;
}

test(&mut c, 213);
print(c);

