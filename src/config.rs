use std:: collections::HashMap;
use std::     convert::TryFrom;
use std::        path::PathBuf;


use std::fs;

use failure  ::Error  ;

use serde_yaml;

use super::*;



/// Internal representation of the yaml config.
///
//  This is a tuple struct to keep the yaml format as concise as possible.
//
#[ derive( Debug, Deserialize, Serialize, Clone, PartialEq ) ]
//
pub struct Config
(
	HashMap< String, Profile >
);


/// A specific profile of mappings from gamepad to mouse-keyboard.
/// A profile can be switched runtime.
//
pub type Profile = HashMap< InputID, Vec< ActionCfg > >;


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
	pub fn profiles( &self ) -> &HashMap< String, Profile >
	{
		&self.0
	}
}



impl TryFrom< PathBuf > for Config
{
	type Error = Error;

	fn try_from( path: PathBuf ) -> Result< Self, Error >
	{
		let s: Self = serde_yaml::from_str( &fs::read_to_string( path )? )?;

		Ok( s )
	}
}


