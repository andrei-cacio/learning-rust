use std::env;

fn fibo(n: i32) -> i32 {
	if n < 2 {
		return n;
	}

	return fibo(n - 1) + fibo(n - 2);
}

fn main() {
	let args = env::args().collect::<Vec<String>>();
	let app_name = &args[0];
	if args.len() < 2 {
		println!("usage {:?} [number]", app_name);
	} else {
		let n = &args[1].parse::<i32>().unwrap();
		println!("{}", fibo(*n));
	}
}
