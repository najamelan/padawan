// Opt in to unstable features expected for Rust 2018
//
#![feature(rust_2018_preview)]

// Opt in to warnings about new 2018 idioms
//
#![warn(rust_2018_idioms)]


use gamepad::*;


fn main()
{

	let mut pad = Pad::new();

	let config  = Config::new().expect( "Could not parse config file" );
	let profile = config.profiles.first().unwrap().get( "swtor_basics" ).unwrap();

	pad.configure( profile );

	let mut all_pads = vec![ pad ];

	Pad::listen( &mut all_pads );

	println!("{:?}", all_pads.first() );

}
