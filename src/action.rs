use std::   fmt::Debug;
use std::str::FromStr;

use ::enigo::{ Key, MouseButton, Enigo, KeyboardControllable, MouseControllable };

// TODO: make one global enigo variable instead of instantiating in every method.


// Generic types for both keyboard and mouse buttons.
//
#[ derive( Debug, Copy, Clone ) ]
pub enum Clickable
{
	Keyboard( Key         ),
	Mouse   ( MouseButton ),
}


impl Clickable
{
	fn press( &self )
	{
		let mut enigo = Enigo::new();

		if      let Clickable::Keyboard( key ) = self { enigo.key_down  ( *key ) }
		else if let Clickable::Mouse   ( key ) = self { enigo.mouse_down( *key ) }
	}


	fn release( &self )
	{
		let mut enigo = Enigo::new();

		if      let Clickable::Keyboard( key ) = self { enigo.key_up  ( *key ) }
		else if let Clickable::Mouse   ( key ) = self { enigo.mouse_up( *key ) }
	}
}


impl std::convert::From< &String > for Clickable
{
	fn from( key: &String ) -> Self
	{
		match key.as_ref()
		{
			"mouse_left"   => Clickable::Mouse   ( MouseButton::Left   ),
			"mouse_middle" => Clickable::Mouse   ( MouseButton::Middle ),
			"mouse_right"  => Clickable::Mouse   ( MouseButton::Right  ),

			"Return"       => Clickable::Keyboard( Key::Return         ),

			_              =>
			{
				match char::from_str( key )
				{
					Ok ( c ) => Clickable::Keyboard( Key::Layout( c ) ),
					Err( e ) => panic!( "Keys must be only one char, got: {:?}", e )
				}

			}
		}
	}
}



// Represents an action that can be attached to a gamepad input event
//
pub trait Action : Debug
{
	fn run( &mut self, state: f32 );
}




// Different Actions we can attach to gamepad input events
//
#[ derive( Debug, Clone ) ] pub struct PressKey     { pub key   : Clickable                                          }
#[ derive( Debug, Clone ) ] pub struct ReleaseKey   { pub key   : Clickable                                          }
#[ derive( Debug, Clone ) ] pub struct Axis2Button  { pub left  : Clickable, pub right: Clickable, pub deadzone: f32 }
#[ derive( Debug, Clone ) ] pub struct ToggleButton { pub button: Clickable, pub state: bool                         }

#[ derive( Debug, Clone ) ] pub struct MouseMapX    { pub pixels: f32                                                }
#[ derive( Debug, Clone ) ] pub struct MouseMapY    { pub pixels: f32                                                }


impl Action for PressKey
{
	fn run( &mut self, _state: f32 ) { self.key.press(); }
}


impl Action for ReleaseKey
{
	fn run( &mut self, _state: f32 ) { self.key.release(); }
}



impl Action for MouseMapX
{
	fn run( &mut self, state: f32 )
	{
		let mut enigo = Enigo::new();

		enigo.mouse_move_relative( (state * state.abs() * self.pixels) as i32, 0 );
	}
}



impl Action for MouseMapY
{
	fn run( &mut self, state: f32 )
	{
		let mut enigo = Enigo::new();

		// The mouse move logic starts with 0 at top left, but moving the thumb stick up is posistive, hence the - to invert things.invert
		//
		enigo.mouse_move_relative( 0, -( state * state.abs() * self.pixels ) as i32 );
	}
}



impl Action for Axis2Button
{
	fn run( &mut self, state: f32 )
	{
		     if state >  self.deadzone { self.left .release(); self.right.press(); }
		else if state < -self.deadzone { self.right.release(); self.left .press(); }

		else
		{
			self.right.release();
			self.left .release();
		}
	}
}



// This will toggle the state of a keyboard or mouse button between up and down on a click on the input button.
// It holds the button down until clicked again.
//
impl Action for ToggleButton
{
	fn run( &mut self, _: f32 )
	{
		if   self.state { self.button.release(); }
		else            { self.button.press  (); }

		self.state = !self.state;
	}
}


