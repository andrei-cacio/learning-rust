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
	if args.len() != 2 {
		println!("Error: Invalid number of arguments\nUsage: {:?} [number]", app_name);
		std::process::exit(1);
	} else {
		let n = args[1].parse::<i32>().unwrap_or_else(|e| {
			println!("Error parsing number '{}': {}", args[1], e);
			std::process::exit(1);
		});
		println!("{}", fibo(n));
	}
}
