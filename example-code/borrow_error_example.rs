let tval: String = "tval";
let mut test_s: & String = &tval;
*test_s = "test_s";
let dubbor: & String = &test_s;

fn test() -> () {
    let mut scope: &String = &"hello";
    {
	    let tval2: String = "tval2";
	    let mut debor: String = *tval2;
	    debor = *tval2;
	    scope = &tval2;
    }
}

test();
print(*test_s);
