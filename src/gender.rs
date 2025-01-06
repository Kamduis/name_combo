//! Provides gender differentiation of a name.




//=============================================================================
// Crates


use std::hash::Hash;
use std::fmt;

#[cfg( feature = "i18n" )] use fluent_templates::Loader;
#[allow( unused )] use log::{error, warn, info, debug};
#[cfg( feature = "serde" )] use serde::{Serialize, Deserialize};
use unic_langid::LanguageIdentifier;

#[cfg( feature = "i18n" )] use crate::DisplayLocale;
#[cfg( feature = "i18n" )] use crate::LOCALES;
use crate::name::NameError;




//=============================================================================
// Enums


/// A subset of possible genders.
#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Clone, Copy, Hash, PartialEq, Eq, Default, Debug )]
pub enum Gender {
	#[default]
	Undefined,
	Male,
	Female,
	Neutral,
	Other,
}

impl Gender {
	/// Returns a slice of all available `Gender`s.
	pub const ALL: &[ Gender; 5 ] = &[
		Self::Undefined,
		Self::Male,
		Self::Female,
		Self::Neutral,
		Self::Other,
	];

	/// Returns the German polite address for a person of the respective gender. If the gender has no respective address, this method returns `None`.
	///
	/// # Error
	/// If the `locale` is not supported, this method returns an error.
	///
	/// # Arguments
	/// * `locale` the locale to use. Currently only English and German are supported.
	pub(crate) fn polite( &self, locale: &LanguageIdentifier ) -> Result<String, NameError> {
		let res = match locale.language.as_str() {
			"en" => match self {
				Self::Male    => "Mister",
				Self::Female  => "Miss",
				Self::Undefined | Self::Neutral | Self::Other => return Err( NameError::NotExpressionable(
					format!( "Gender has no polite address: {}", self )
				) ),
			}
			"de" => match self {
				Self::Male    => "Herr",
				Self::Female  => "Frau",
				Self::Undefined | Self::Neutral | Self::Other => return Err( NameError::NotExpressionable(
					format!( "Gender has no polite address: {}", self )
				) ),
			}
			_ => return Err( NameError::LangNotSupported( locale.to_string() ) ),
		};

		Ok( res.to_string() )
	}

	/// Returns the symbol representing the gender of `self`.
	pub fn to_symbol( &self ) -> String {
		let res = match self {
			Self::Male    => "♂",
			Self::Female  => "♀",
			Self::Undefined | Self::Neutral => "⚪",
			Self::Other   => "⚧",
		};

		res.to_string()
	}
}

impl fmt::Display for Gender {
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		let res = match self {
			Self::Undefined => "undefined",
			Self::Male    => "male",
			Self::Female  => "female",
			Self::Neutral => "neutral",
			Self::Other   => "other",
		};

		write!( f, "{}", res )
	}
}

#[cfg( feature = "i18n" )]
impl DisplayLocale for Gender {
	fn to_string_locale( &self, locale: &LanguageIdentifier ) -> String {
		match self {
			Self::Undefined => LOCALES.lookup( locale, "undefined" ),
			Self::Male    => LOCALES.lookup( locale, "male" ),
			Self::Female  => LOCALES.lookup( locale, "female" ),
			Self::Neutral => LOCALES.lookup( locale, "neutral" ),
			Self::Other   => LOCALES.lookup( locale, "other" ),
		}
	}
}




//=============================================================================
// Testing


#[cfg( test )]
mod tests {
	use super::*;

	#[test]
	fn gender_all() {
		assert!( Gender::ALL.iter().count() > 0 );
	}

	#[test]
	fn gender_title() {
		use unic_langid::langid;

		const US_ENGLISH: LanguageIdentifier = langid!( "en-US" );
		const GERMAN: LanguageIdentifier = langid!( "de-DE" );

		assert_eq!( Gender::Male.polite( &US_ENGLISH ).unwrap(), "Mister".to_string() );
		assert_eq!( Gender::Female.polite( &US_ENGLISH ).unwrap(), "Miss".to_string() );
		assert_eq!( Gender::Male.polite( &GERMAN ).unwrap(), "Herr".to_string() );
		assert_eq!( Gender::Female.polite( &GERMAN ).unwrap(), "Frau".to_string() );
		assert!( Gender::Neutral.polite( &GERMAN ).is_err() );
		assert!( Gender::Other.polite( &GERMAN ).is_err() );
	}

	#[test]
	fn gender_symbol() {
		assert_eq!( Gender::Male.to_symbol(), "♂".to_string() );
		assert_eq!( Gender::Female.to_symbol(), "♀".to_string() );
		assert_eq!( Gender::Neutral.to_symbol(), "⚪".to_string() );
		assert_eq!( Gender::Other.to_symbol(), "⚧".to_string() );
	}

	#[test]
	fn gender_text() {
		assert_eq!( Gender::Male.to_string(), "male".to_string() );
		assert_eq!( Gender::Female.to_string(), "female".to_string() );
		assert_eq!( Gender::Neutral.to_string(), "neutral".to_string() );
		assert_eq!( Gender::Other.to_string(), "other".to_string() );
	}
}
