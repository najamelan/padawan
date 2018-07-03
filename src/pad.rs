// The Pad struct is a frontend to the gilrs library.
// This allows us to present the ideal interface to our application, independant of the external library.
//


use std::        time::Duration;
use std:: collections::HashMap ;
use std::     convert::From    ;
use std::       slice::Iter    ;
use std::      thread          ;

use super::*;

use gilrs::{ Gilrs, Event, EventType as GEventType, Button as GButton, Axis as GAxis };

use self::InputID::*;



/// Represents the actual controller with all of it's buttons.
/// You can tell it to attach mapping configurations to events from buttons.
///
#[ derive( Debug ) ]
//
pub struct Pad
{
	// We have an object for each input on the gamepad
	//
	inputs: HashMap< InputID, Input >
}


impl Pad
{
	const SLEEP_MS: u64 = 5;


	pub fn new() -> Self
	{
		let mut inputs = HashMap::new();


		for &v in InputID::variants()
		{
			inputs.insert( v, Input::new( v ) );
		}


		Pad{ inputs: inputs }
	}



	/// Listens to the event loop of gilrs. This will loop and will not return. Eg. Blocks the current thread.
	///
	/// TODO: make a working system for when there is more than 1 gamepad connected.
	///
	pub fn listen( pads: &mut Vec<Pad> )
	{
		let mut gpads   = Gilrs::new().unwrap();
		let     pad     = pads.first_mut().unwrap();


		loop
		{
			while let Some( Event { id: _, event, time: _ } ) = gpads.next_event()
			{
				pad.process_event( event.into() );
			}


			pad.process_event( EventType::NoChange );


			thread::sleep( Duration::from_millis ( Self::SLEEP_MS ) );
		}
	}



	pub fn process_event( &mut self, event: EventType )
	{
		match event
		{
			EventType::ButtonPressed ( button    ) => self.input_mut( button.into() ).process_event( event ),
			EventType::ButtonReleased( button    ) => self.input_mut( button.into() ).process_event( event ),
			EventType::ButtonChanged ( button, _ ) => self.input_mut( button.into() ).process_event( event ),
			EventType::ButtonRepeated( button    ) => self.input_mut( button.into() ).process_event( event ),
			EventType::AxisChanged   ( axis  , _ ) => self.input_mut( axis  .into() ).process_event( event ),
			EventType::NoChange                    => for (_, i) in &mut self.inputs { i.process_event( event ) },
			EventType::Connected                   => (),
			EventType::Disconnected                => (),
			EventType::Dropped                     => (),
		};
	}



	pub fn input( &self, id: InputID ) -> &Input
	{
		self.inputs.get( &id ).unwrap()
	}



	pub fn input_mut( &mut self, id: InputID ) -> &mut Input
	{
		self.inputs.get_mut( &id ).unwrap()
	}



	/// Map configuration to actual event handlers on our inputs.
	///
	pub fn configure( &mut self, profile: &Profile )
	{
		for (input, actions) in profile { self.map_config( input.clone(), actions ) }
	}



	/// Map configuration to actual event handlers on our inputs.
	//
	#[inline]
	//
	pub fn map_config( &mut self, input: InputID, actions: &Vec< ActionCfg > )
	{
		for action in actions
		{
			match action
			{
				ActionCfg::Button( which  ) => self.map_button ( input, Clickable::from( which ) ),
				ActionCfg::Toggle( which  ) => self.map_toggle ( input, Clickable::from( which ) ),
				ActionCfg::MouseX( pixels ) => self.map_mouse_x( input, *pixels                  ),
				ActionCfg::MouseY( pixels ) => self.map_mouse_y( input, *pixels                  ),


				ActionCfg::Axis2Button( left, right, deadzone ) =>

					self.map_axis2button( input, Clickable::from( left ), Clickable::from( right ), *deadzone ),
			};
		}

	}



	/// Map a button on the gamepad to a button on keyboard or mouse.
	///
	pub fn map_button( &mut self, input_id: InputID, key: Clickable )
	{
		let act   = PressKey  { key: key };
		let act2  = ReleaseKey{ key: key };

		let trig  = Trigger::OnDown( Box::new( act  ) );
		let trig2 = Trigger::OnUp  ( Box::new( act2 ) );

		let input = self.input_mut( input_id );

		input.add_trigger( trig  );
		input.add_trigger( trig2 );
	}



	/// Toggle a button on the keyboard or mouse.
	/// This will toggle the state of a keyboard or mouse button between up and down on a click on the input button.
	/// It holds the button down until clicked again.
	///
	pub fn map_toggle( &mut self, input_id: InputID, key: Clickable )
	{
		let mt    = ToggleButton{ button: key, state: false };
		let tt    = Trigger::OnUp( Box::new( mt ) );

		let input = self.input_mut( input_id );


		input.add_trigger( tt );
	}



	/// Map an input to the mouse movement (X-axis). This is most useful with the sticks on the gamepad, but it can be used also
	/// with other buttons. Using four buttons you can get all directions of the mouse. Use the pixels parameter with a negative value
	/// to reverse the direction.
	/// @param pixels defines how fast to move the mouse. You can consider 4.0 a default value. Adapt from there to your needs.
	///
	pub fn map_mouse_x( &mut self, input_id: InputID, pixels: f32 )
	{
		let input = self.input_mut( input_id );
		let mm    = MouseMapX{ pixels: pixels };

		let tr    = Trigger::OnChange  ( Box::new( mm.clone() ) );
		let tr2   = Trigger::OnNoChange( Box::new( mm         ) );

		input.add_trigger( tr  );
		input.add_trigger( tr2 );
	}



	/// See map_mouse_x. Everything works the same except it's on the Y-axis.
	///
	pub fn map_mouse_y( &mut self, input_id: InputID, pixels: f32 )
	{
		let input = self.input_mut( input_id );
		let mm    = MouseMapY{ pixels: pixels };

		let tr    = Trigger::OnChange  ( Box::new( mm.clone() ) );
		let tr2   = Trigger::OnNoChange( Box::new( mm         ) );

		input.add_trigger( tr  );
		input.add_trigger( tr2 );
	}


	// This allows to connect a thumb stick to 4 buttons, for example movement with keys adws
	// We couple 2 buttons to one axis, eg. a and d on the X-axis. A will be held down while
	// the stick is to the left, D will be held down when the stick is to the right.
	//
	//
	pub fn map_axis2button( &mut self, input_id: InputID, left: Clickable, right: Clickable, deadzone: f32 )
	{
		let input = self.input_mut( input_id );

		let act   = Axis2Button{ left: left, right: right, deadzone: deadzone };
		let trig  = Trigger::OnChange( Box::new( act ) );

		input.add_trigger( trig );
	}
}




/// The representation of any input (button) on the gamepad
///
#[ derive( Debug ) ]
//
pub struct Input
{
	_id        : InputID        ,
	triggers   : Vec< Trigger > ,
	state      : f32            ,
	old_state  : f32            ,
}



impl Input
{
	pub fn new( id: InputID ) -> Self
	{
		Self
		{
			_id        : id         ,
			triggers   : Vec::new() ,
			state      : 0.0        ,
			old_state  : 0.0        ,
		}
	}



	pub fn process_event( &mut self, event: EventType )
	{
		let os = &mut self.old_state;
		let st = &mut self.state;

		for trigger in &mut self.triggers
		{
			match event
			{
				EventType::ButtonPressed ( .. )      => if let Trigger::OnDown    ( act ) = trigger { *os = *st; *st = 1.0; act.run( *st ) },
				EventType::ButtonReleased( .. )      => if let Trigger::OnUp      ( act ) = trigger { *os = *st; *st = 0.0; act.run( *st ) },
				EventType::ButtonRepeated( .. )      => (),

				EventType::NoChange                  => if let Trigger::OnNoChange( act ) = trigger { act.run( *st ) },

				EventType::ButtonChanged( _, state ) => if let Trigger::OnChange  ( act ) = trigger { *os = *st; *st = state; act.run( *st ) },
				EventType::AxisChanged  ( _, state ) => if let Trigger::OnChange  ( act ) = trigger { *os = *st; *st = state; act.run( *st ) },

				_ => ()
			};

		}
	}



	#[inline]
	pub fn set_state( &mut self, state: f32 )
	{
		self.old_state = self.state;
		self.state     = state     ;
	}



	pub fn add_trigger( &mut self, trigger: Trigger )
	{
		self.triggers.push( trigger );
	}
}



#[ derive( Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize ) ]
//
/// Different buttons present on standard gamepad
///
pub enum InputID
{
	// Action Pad
	//
	South         ,
	East          ,
	North         ,
	West          ,

	// Triggers
	//
	LeftTrigger   ,
	LeftTrigger2  ,
	RightTrigger  ,
	RightTrigger2 ,

	// Menu Pad
	//
	Select        ,
	Start         ,
	Mode          ,

	// D-Pad
	//
	DPadUp        ,
	DPadDown      ,
	DPadLeft      ,
	DPadRight     ,

	// Sticks clicked as button
	//
	LeftThumb     ,
	RightThumb    ,

	// The movement of the sticks
	//
	LeftStickX    ,
	LeftStickY    ,
	RightStickX   ,
	RightStickY   ,

	Unknown       ,
}




// Different event types
//
#[ derive( Debug ) ]
//
pub enum Trigger
{
	OnDown    ( Box< dyn Action > ),
	OnUp      ( Box< dyn Action > ),
	OnChange  ( Box< dyn Action > ),
	OnNoChange( Box< dyn Action > ),
}



/// Gamepad event.
//
#[ derive( Debug, Clone, Copy, PartialEq ) ]
//
pub enum EventType
{
	/// Some button on gamepad has been pressed.
	//
	ButtonPressed( InputID ),

	/// This event can be generated by [`ev::Repeat`](filter/struct.Repeat.html) event filter.
	//
	ButtonRepeated( InputID ),

	/// Previously pressed button has been released.
	//
	ButtonReleased( InputID ),

	/// Value of button has changed. Value can be in range [0.0, 1.0].
	//
	ButtonChanged( InputID, f32 ),

	/// Value of axis has changed. Value can be in range [-1.0, 1.0].
	//
	AxisChanged( InputID, f32 ),

	/// An event to allow triggering at a regular intervall. You can attach event handlers to these non-events.
	/// Notably works to make sticks fire permanently when they are not on 0.
	//
	NoChange,

	/// Gamepad has been connected. If gamepad's UUID doesn't match one of disconnected gamepads,
	/// newly connected gamepad will get new ID. This event is also emitted when creating `Gilrs`
	/// for every gamepad that was already connected.
	//
	Connected,

	/// Gamepad has been disconnected. Disconnected gamepad will not generate any new events.
	//
	Disconnected,

	/// There was an `Event`, but it was dropped by one of filters. You should ignore it.
	//
	Dropped,
}



// -----------------------------------------------------------------------------
// -
// - BOILERPLATE
// -
// - ---------------------------------------------------------------------------
// -


impl InputID
{
	pub fn variants() -> Iter< 'static,  InputID >
	{
		static IDS: [ InputID; 22 ] =
		[
			// Action Pad
			//
			South         ,
			East          ,
			North         ,
			West          ,

			// Triggers
			//
			LeftTrigger   ,
			LeftTrigger2  ,
			RightTrigger  ,
			RightTrigger2 ,

			// Menu Pad
			//
			Select        ,
			Start         ,
			Mode          ,

			// D-Pad
			//
			DPadUp        ,
			DPadDown      ,
			DPadLeft      ,
			DPadRight     ,

			// Sticks clicked as button
			//
			LeftThumb     ,
			RightThumb    ,

			// The movement of the sticks
			//
			LeftStickX    ,
			LeftStickY    ,
			RightStickX   ,
			RightStickY   ,

			Unknown
		];


		IDS.into_iter()
	}
}



/// Translate GEventType to EventType
//
impl From< GEventType > for EventType
{
	fn from( event_type: GEventType ) -> Self
	{
		match event_type
		{
			GEventType::ButtonPressed ( input       , _ ) => EventType::ButtonPressed ( input.into()        ),
			GEventType::ButtonRepeated( input       , _ ) => EventType::ButtonRepeated( input.into()        ),
			GEventType::ButtonReleased( input       , _ ) => EventType::ButtonReleased( input.into()        ),
			GEventType::ButtonChanged ( input, value, _ ) => EventType::ButtonChanged ( input.into(), value ),
			GEventType::AxisChanged   ( input, value, _ ) => EventType::AxisChanged   ( input.into(), value ),
			GEventType::Connected                         => EventType::Connected                            ,
			GEventType::Disconnected                      => EventType::Disconnected                         ,
			GEventType::Dropped                           => EventType::Dropped                              ,
		}
	}

}



/// Translate gilrs buttons to our InputID
//
impl From< GButton > for InputID
{
	fn from( button: GButton ) -> Self
	{
		match button
		{
			GButton::North         => InputID::North         ,
			GButton::East          => InputID::East          ,
			GButton::South         => InputID::South         ,
			GButton::West          => InputID::West          ,

			GButton::LeftTrigger   => InputID::LeftTrigger   ,
			GButton::LeftTrigger2  => InputID::LeftTrigger2  ,
			GButton::RightTrigger  => InputID::RightTrigger  ,
			GButton::RightTrigger2 => InputID::RightTrigger2 ,

			GButton::Select        => InputID::Select        ,
			GButton::Start         => InputID::Start         ,
			GButton::Mode          => InputID::Mode          ,

			GButton::LeftThumb     => InputID::LeftThumb     ,
			GButton::RightThumb    => InputID::RightThumb    ,

			GButton::DPadUp        => InputID::DPadUp        ,
			GButton::DPadDown      => InputID::DPadDown      ,
			GButton::DPadLeft      => InputID::DPadLeft      ,
			GButton::DPadRight     => InputID::DPadRight     ,

			GButton::Unknown       => InputID::Unknown       ,
			GButton::C             => InputID::Unknown       ,
			GButton::Z             => InputID::Unknown       ,
		}
	}
}



impl From< GAxis > for InputID
{
	fn from( axis: GAxis ) -> Self
	{
		match axis
		{
			GAxis::LeftStickX  => InputID::LeftStickX  ,
			GAxis::LeftStickY  => InputID::LeftStickY  ,
			GAxis::RightStickX => InputID::RightStickX ,
			GAxis::RightStickY => InputID::RightStickY ,

			GAxis::Unknown     => InputID::Unknown     ,
			GAxis::LeftZ       => InputID::Unknown     ,
			GAxis::RightZ      => InputID::Unknown     ,
			GAxis::DPadX       => InputID::Unknown     ,
			GAxis::DPadY       => InputID::Unknown     ,
		}
	}
}
