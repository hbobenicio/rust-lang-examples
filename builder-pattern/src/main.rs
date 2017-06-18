extern crate builder_pattern;

use builder_pattern::hero::*;

fn main() {
	let hero = HeroBuilder::new()
		.name("Conan".to_string())
		.atk(10)
		.def(8)
		.speed(4)
		.build();
	println!("{:?}", hero);
}
