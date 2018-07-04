// Opt in to unstable features expected for Rust 2018
//
#![feature(rust_2018_preview)]

// Opt in to warnings about new 2018 idioms
//
#![warn(rust_2018_idioms)]

#![feature(try_from)]



// TODO:
//   - test with a realistic profile in swtor
//   - provide more possible actions
//   - test command line parameters
//   - support multiple pads
//   - document
//   - error handling, remove all unwraps
//   - unit test
//   - publish
//

use std::convert::TryFrom;
use std::path::PathBuf;
use failure::Error;
use std::env;


use libpadawan::*;
use clap::{ App, load_yaml };

fn main()
{

	// Command line options:
	//
	// The YAML file is found relative to the current file, similar to how modules are found
	//
	let yaml    = load_yaml!( "clap.yml" );
	let matches = App::from_yaml( yaml ).get_matches();

	// Gets a value for config if supplied by user, or defaults to "default.conf"
	//
	let cfg_file = matches.value_of( "config" ).unwrap_or( "config.yml" );
	println!( "Value for config: {}", cfg_file );

	let prof_cfg = matches.value_of( "profile" ).unwrap_or( "swtor_basics" );
	println!( "Value for config: {}", prof_cfg );


	//-----------------------------------------------------------------------------

	let config =

		Config::try_from
		(

			 abs_path( cfg_file )
			.expect  ( &format!( "Could not find configuration file: {}", cfg_file ) )

		).expect( "Could not parse config file" )
	;




	let mut     pad = Gamepad::new();
	let     profile = config.profiles().get( prof_cfg ).expect( &format!( "Profile <{}> not found in configuration file!", prof_cfg ) );

	pad.configure( profile );

	let mut all_pads = vec![ pad ];

	Gamepad::listen( &mut all_pads );

	println!("{:?}", all_pads.first() );
}





// Takes a relative path and returns an absolute one, searching in the cwd, then relative to the executable file.
//
pub fn abs_path( relative: &str ) -> Result< PathBuf, Error >
{
	let entry = env::current_dir()?.as_path().join( relative );

	if entry.exists()

		{ return Ok( entry.canonicalize()? ) }


	let exe    = env::current_exe()?;
	let parent = exe.parent().ok_or( std::io::Error::new( std::io::ErrorKind::NotFound, "Program executable has no parent directory" ) )?;

   // Canonicalize will throw an error if the file doesn't exist
   //
   Ok( parent.join( relative ).canonicalize()? )
}

