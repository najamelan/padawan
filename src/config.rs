use std::collections::HashMap;
use std::path::PathBuf;
use std::env;
use std::fs;
use std;

use failure  ::Error  ;

use serde_yaml;

use super::*;




#[ derive( Debug, Deserialize, Serialize, Clone, PartialEq ) ]
//
pub struct Config
{
	pub profiles: Vec< HashMap< String, Vec< MappingCfg > > >
}


#[ derive( Debug, Deserialize, Serialize, Clone, PartialEq ) ]
//
pub struct MappingCfg
{
	pub input : InputID,
	pub map   : Vec< ActionCfg >,
}



#[ derive( Debug, Deserialize, Serialize, Clone, PartialEq ) ]
//
pub enum ActionCfg
{
	Button     ( String              ),
	Toggle     ( String              ),
	MouseX     (    f32              ),
	MouseY     (    f32              ),
	Axis2Button( String, String, f32 ),
}




impl Config
{
	pub fn new() -> Result< Self, Error >
	{
		let file = abs_path( "config.yml" )?;

		let s: Self = serde_yaml::from_str( &fs::read_to_string( file )? )?;

		Ok( s )
	}
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
