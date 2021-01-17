fn hello_world() -> () {
	print("Hello World!");
}

let c: f32 = 22.2;

fn foo(bar: i32) -> bool {
	hello_world();
	while true {
        	let a: f32 = 10.2 + c;
		let b: & f32 = &c;
        	if bar == 21 {
			return true;
        	} else {
			foo(21);
            		return false;
        	}
    	}
	return false;
}

print(foo(10));

