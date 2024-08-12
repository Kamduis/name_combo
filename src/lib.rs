// Replace crate links with internal links when creating documentation with `cargo`.
//! [`DisplayLocale`]: DisplayLocale
//! [`serde`]: serde
// File links are not supported by rustdoc.
//! [LICENSE-APACHE]: https://github.com/Kamduis/name_combo/blob/master/LICENSE-APACHE
//! [LICENSE-MIT]: https://github.com/Kamduis/name_combo/blob/master/LICENSE-MIT
//!
//! <style>
//! .rustdoc-hidden { display: none; }
//! </style>
#![doc = include_str!( "../README.md" )]




//=============================================================================
// Crates


use std::hash::Hash;
use std::fmt;
use std::str::FromStr;

#[cfg( feature = "i18n" )] use fluent_templates::Loader;
#[allow( unused )] use log::{error, warn, info, debug};
#[cfg( feature = "serde" )] use serde::{Serialize, Deserialize};
use thiserror::Error;
use unic_langid::LanguageIdentifier;




//=============================================================================
// Errors


#[derive( Error, PartialEq, Debug )]
pub enum NameError {
	#[error( "This grammatical case is illegal." )]
	IllegalCase,

	#[error( "This name combo is illegal." )]
	IllegalCombo,

	#[error( "Name element missing: `{0}`" )]
	MissingNameElement( String ),

	#[error( "Name cannot be expressed: `{0}`" )]
	NotExpressionable( String ),

	#[error( "Language not yet supported: `{0}`" )]
	LangNotSupported( String ),
}




//=============================================================================
// Traits


/// Providing a localized `.to_string()`: `.to_string_locale()`.
///
/// This Trait is only available, if the **`i18n`** feature has been enabled.
#[cfg( feature = "i18n" )]
pub trait DisplayLocale: fmt::Display {
	/// Returns the localized string representation of `self`.
	///
	/// The standard implementation ignores `locale` and returns the same string as `.to_string()`.
	#[allow( unused_variables )]
	fn to_string_locale( &self, locale: &LanguageIdentifier ) -> String {
		self.to_string()
	}
}




//=============================================================================
// Internationalization


#[cfg( feature = "i18n" )]
fluent_templates::static_loader! {
	static LOCALES = {
		// The directory of localisations and fluent resources.
		locales: "./locales",

		// The language to falback on if something is not present.
		fallback_language: "en-US",
	};
}




//=============================================================================
// Helper functions


/// Creating initials from `text` by only taking the first letter of each word and adding a dot after it.
///
/// Bsp. "Thomas von Würzinger" => "T. v. W."
fn initials( text: &str ) -> String {
	if text.is_empty() {
		return "".to_string();
	}

	text.split( ' ' )
		.map( |x| format!( "{}.", x.chars().next().unwrap() ) )
		.collect::<Vec<String>>()
		.join( " " )
}


/// Adding letters to `text` depending on the grammatical case. `text` is assumed to be of the nominative case.
///
/// # Arguments
/// * `text` the text to modify depending on grammatical case.
/// * `case` the grammatical case.
/// * `locale` the locale to use the grammatical rules of. Currently only English and German are supported.
fn add_case_letter( text: &str, case: GrammaticalCase, locale: &LanguageIdentifier ) -> Result<String, NameError> {
	// In the currently supported languages (English and German), only the genetive case is changing the writing of a name.
	let GrammaticalCase::Genetive = case else {
		return Ok( text.to_string() );
	};

	if text.is_empty() {
		return Ok( "".to_string() );
	}

	let glyph_last = text.chars()
		.last().unwrap()
		.to_lowercase()
		.to_string();

	let appendix = match locale.language.as_str() {
		"en" => match glyph_last.as_str() {
			"s" => "'",
			_ => "'s",
		},
		"de" => match glyph_last.as_str() {
			"s" | "ß" | "z" | "x" => "'",
			_ => "s",
		},
		_ => return Err( NameError::LangNotSupported( locale.to_string() ) ),
	};

	Ok( format!( "{}{}", text, appendix ) )
}




//=============================================================================
// Enums


/// The different grammatical cases.
#[derive( Clone, Copy, PartialEq, Eq, Debug )]
pub enum GrammaticalCase {
	Nominative,
	Genetive,
	Dative,
	Accusative,
}

impl FromStr for GrammaticalCase {
	type Err = NameError;

	fn from_str( s: &str ) -> Result<Self, Self::Err> {
		let res = match s.to_lowercase().as_str() {
			"nominative" => Self::Nominative,
			"genetive" | "s" => Self::Genetive,
			"dative" => Self::Dative,
			"accusative" => Self::Accusative,
			_ => {
				error!( "{:?} is not a supported grammatical case.", s );
				return Err( NameError::IllegalCase );
			},
		};

		Ok( res )
	}
}


/// A subset of possible genders.
#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Clone, Copy, Hash, PartialEq, Eq, Debug )]
pub enum Gender {
	Male,
	Female,
	Neutral,
	Other,
}

impl Gender {
	/// Returns the German polite address for a person of the respective gender. If the gender has no respective address, this method returns `None`.
	///
	/// # Error
	/// If the `lacle` is not supported, this method returns an error.
	///
	/// # Arguments
	/// * `locale` the locale to use. Currently only English and German are supported.
	fn polite( &self, locale: &LanguageIdentifier ) -> Result<String, NameError> {
		let res = match locale.language.as_str() {
			"en" => match self {
				Self::Male    => "Mister",
				Self::Female  => "Miss",
				Self::Neutral | Self::Other => return Err( NameError::NotExpressionable(
					format!( "Gender has no polite address: {}", self )
				) ),
			}
			"de" => match self {
				Self::Male    => "Herr",
				Self::Female  => "Frau",
				Self::Neutral | Self::Other => return Err( NameError::NotExpressionable(
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
			Self::Neutral => "⚪",
			Self::Other   => "⚧",
		};

		res.to_string()
	}
}

impl fmt::Display for Gender {
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		let res = match self {
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
			Self::Male    => LOCALES.lookup( locale, "male" ),
			Self::Female  => LOCALES.lookup( locale, "female" ),
			Self::Neutral => LOCALES.lookup( locale, "neutral" ),
			Self::Other   => LOCALES.lookup( locale, "other" ),
		}
	}
}


/// The possible combination of names.
#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Clone, Copy, PartialEq, Eq, Debug )]
pub enum NameCombo {
	/// This represents the standard (german) name combination of first name and surname. Bsp.: "Penelope von Würzinger"
	Name,

	/// The full name. Bsp.: "Penelope Karin von Würzinger geb. Stauff"
	Fullname,

	/// The first forename. Bsp.: "Thomas"
	Firstname,

	/// All forenames. Bsp.: "Thomas Jakob"
	Forenames,

	/// Only the full surname. This includes all name predicates. Bsp.: "von Würzinger"
	Surname,

	/// Only the title (academic title or something else). Bsp.: "Dr."
	Title,

	/// Title with first forename and surname. Bsp.: "Dr. Penelope von Würzinger"
	TitleName,

	/// Title with first forename. Bsp.: "Dr. Penelope"
	TitleFirstname,

	/// Title with surname. Bsp.: "Dr. von Würzinger"
	TitleSurname,

	/// Title with full name. Bsp.: "Dr. Penelope Karin von Würzinger geb. Stauff"
	TitleFullname,

	/// Only the polite address. Bsp.: "Herr"
	Polite,

	/// Polite with first forename and surname. Bsp.: "Herr Thomas von Würzinger"
	PoliteName,

	/// Polite with first forename. Bsp.: "Frau Penelope"
	PoliteFirstname,

	/// Polite with surname. Bsp.: "Herr von Würzinger"
	PoliteSurname,

	/// Polite with full name. Bsp.: "Frau Penelope Karin von Würzinger geb. Stauff"
	PoliteFullname,

	/// Polite with title, first forename and surname. Bsp.: "Frau Dr. Penelope von Würzinger"
	PoliteTitleName,

	/// Bsp.: Hauptkommissar
	Rank,

	/// Bsp.: Herr Hauptkommissar
	PoliteRank,

	/// Bsp.: Hauptkommissar Thomas von Würzinger
	RankName,

	/// Bsp.: Hauptkommissar Thomas
	RankFirstname,

	/// Bsp.: Hauptkommissar von Würzinger
	RankSurname,

	/// Bsp.: Majorin Penelope Karin von Würzinger geb. Stauff
	RankFullname,

	/// Bsp.: Majorin Dr. Penelope von Würzinger
	RankTitleName,

	/// Bsp.: Würzi
	Nickname,

	/// Bsp.: Thomas Würzi
	FirstNickname,

	/// Bsp.: Würzi von Würzinger
	NickSurname,

	/// Only the honorific name. Bsp.: "Starke", "Große", "Dunkle"
	Honor,

	/// Honorific name with article. Bsp.: "Der Starke", "Die Große"
	Honortitle,

	/// Honor with first forename. Bsp.: "Penelope die Große"
	FirstHonorname,

	/// Typical antique roman woman's name: Bsp.: Iunia Prima (feminized surname [father's name] Cognomen).
	DuaNomina,

	/// Typical antique roman man's name: Bsp.: Gaius Julius Caeser (firstname surname [father's name] Cognomen).
	TriaNomina,

	/// The supername. Bsp.: Würzt-das-Essen
	Supername,

	/// Firstname and supername. Bsp.: Thomas Würzt-das-Essen
	FirstSupername,

	/// Name with supername between forename and surname. Bsp.: Thomas Würzt-das-Essen von Würzinger
	SuperName,

	/// Polite form of supername. Bsp.: Herr Würzt-das-Essen
	PoliteSupername,

	/// Supername with rank. Bsp.: Hauptkommissar Würzt-das-Essen
	RankSupername,

	/// Initials of firstname and surname. Bsp.: P. v. W.
	Initials,

	/// Initials of all forenames with title and surname. Bsp.: Dr. P. K. v. W.
	InitialsFull,

	/// Surname with initials of forenames (e.g. for nameplates). Bsp.: Dr. P. K. v. Würzinger
	Sign,

	/// Surname first to have a sensible way of alphabetically ordering names. Bsp.: Würzinger, Penelope von
	OrderedName,

	/// Like `Ordered`, only that the forenames are ignored. Bsp.: Würzinger, von
	OrderedSurname,

	/// Like `orderedName`, only with title added. Bsp.: Würzinger, Dr. Penelope von
	OrderedTitleName,
}

impl FromStr for NameCombo {
	type Err = NameError;

	fn from_str( s: &str ) -> Result<Self, Self::Err> {
		let res = match s {
			"Name" => Self::Name,
			"Fullname" => Self::Fullname,
			"Firstname" => Self::Firstname,
			"Forenames" => Self::Forenames,
			"Surname" => Self::Surname,
			"Title" => Self::Title,
			"TitleName" => Self::TitleName,
			"TitleFirstname" => Self::TitleFirstname,
			"TitleSurname" => Self::TitleSurname,
			"TitleFullname" => Self::TitleFullname,
			"Polite" => Self::Polite,
			"PoliteName" => Self::PoliteName,
			"PoliteFirstname" => Self::PoliteFirstname,
			"PoliteSurname" => Self::PoliteSurname,
			"PoliteFullname" => Self::PoliteFullname,
			"PoliteTitleName" => Self::PoliteTitleName,
			"Rank" => Self::Rank,
			"PoliteRank" => Self::PoliteRank,
			"RankName" => Self::RankName,
			"RankFirstname" => Self::RankFirstname,
			"RankSurname" => Self::RankSurname,
			"RankFullname" => Self::RankFullname,
			"RankTitleName" => Self::RankTitleName,
			"Nickname" => Self::Nickname,
			"FirstNickname" => Self::FirstNickname,
			"NickSurname" => Self::NickSurname,
			"Honor" => Self::Honor,
			"Honortitle" => Self::Honortitle,
			"FirstHonorname" => Self::FirstHonorname,
			"DuaNomina" => Self::DuaNomina,
			"TriaNomina" => Self::TriaNomina,
			"Supername" => Self::Supername,
			"FirstSupername" => Self::FirstSupername,
			"SuperName" => Self::SuperName,
			"PoliteSupername" => Self::PoliteSupername,
			"RankSupername" => Self::RankSupername,
			"Initials" => Self::Initials,
			"InitialsFull" => Self::InitialsFull,
			"Sign" => Self::Sign,
			"OrderedName" => Self::OrderedName,
			"OrderedSurname" => Self::OrderedSurname,
			"OrderedTitleName" => Self::OrderedTitleName,
			_ => {
				error!( "{:?} is not a supported name combination.", s );
				return Err( NameError::IllegalCombo );
			},
		};

		Ok( res )
	}
}




//=============================================================================
// Structs


/// The different names of a person that can be combined in various ways.
#[cfg_attr( feature = "serde", derive( Serialize, Deserialize ) )]
#[derive( Clone, Hash, PartialEq, Eq, Default, Debug )]
pub struct Names {
	#[cfg_attr( feature = "serde", serde( default ) )]
	forenames: Vec<String>,

	#[cfg_attr( feature = "serde", serde( default ) )]
	predicate: Option<String>,

	#[cfg_attr( feature = "serde", serde( default ) )]
	surname: Option<String>,

	#[cfg_attr( feature = "serde", serde( default ) )]
	birthname: Option<String>,

	#[cfg_attr( feature = "serde", serde( default ) )]
	title: Option<String>,

	#[cfg_attr( feature = "serde", serde( default ) )]
	rank: Option<String>,

	#[cfg_attr( feature = "serde", serde( default ) )]
	nickname: Option<String>,

	#[cfg_attr( feature = "serde", serde( default ) )]
	honorname: Option<String>,

	#[cfg_attr( feature = "serde", serde( default ) )]
	supername: Option<String>,

	#[cfg_attr( feature = "serde", serde( default ) )]
	gender: Option<Gender>,
}

impl Names {
	/// Create a new `Names`. No name is actually being set.
	pub fn new() -> Self {
		Self::default()
	}

	/// Set the forenames.
	pub fn with_forenames( mut self, names: &[&str] ) -> Self {
		self.forenames = names.iter().map( |x| x.to_string() ).collect();
		self
	}

	/// Set the predicate of a possible surname.
	pub fn with_predicate( mut self, name: &str ) -> Self {
		self.predicate = Some( name.to_string() );
		self
	}

	/// Set the surname.
	pub fn with_surname( mut self, name: &str ) -> Self {
		self.surname = Some( name.to_string() );
		self
	}

	/// Set the birthname.
	pub fn with_birthname( mut self, name: &str ) -> Self {
		self.birthname = Some( name.to_string() );
		self
	}

	/// Set the title.
	pub fn with_title( mut self, title: &str ) -> Self {
		self.title = Some( title.to_string() );
		self
	}

	/// Set the rank.
	pub fn with_rank( mut self, rank: &str ) -> Self {
		self.rank = Some( rank.to_string() );
		self
	}

	/// Set the nickname.
	pub fn with_nickname( mut self, name: &str ) -> Self {
		self.nickname = Some( name.to_string() );
		self
	}

	/// Set the honorname.
	pub fn with_honorname( mut self, name: &str ) -> Self {
		self.honorname = Some( name.to_string() );
		self
	}

	/// Set the supername.
	pub fn with_supername( mut self, name: &str ) -> Self {
		self.supername = Some( name.to_string() );
		self
	}

	/// Set the gender.
	pub fn with_gender( mut self, gender: &Gender ) -> Self {
		self.gender = Some( *gender );
		self
	}

	/// Return the `Gender`.
	pub fn gender( &self ) -> &Option<Gender> {
		&self.gender
	}

	/// Returns all forenames.
	pub fn forenames( &self ) -> &Vec<String> {
		&self.forenames
	}

	/// Returns all forenames as a string. Bsp. "Thomas Jakob". If no forename is given, this returns `None`.
	fn forenames_string( &self ) -> Result<String, NameError> {
		if self.forenames.is_empty() {
			return Err( NameError::MissingNameElement( "forenames".to_string() ) );
		}
		Ok( self.forenames.join( " " ) )
	}

	/// Returns the first forename. If no forenames are given, this method returns `None`.
	pub fn firstname( &self ) -> Option<&str> {
		self.forenames.first().map( |x| x.as_str() )
	}

	/// Returns the first forename. If no forenames are given, this method returns `None`.
	fn firstname_res( &self ) -> Result<&str, NameError> {
		self.forenames.first().map( |x| x.as_str() ).ok_or( NameError::MissingNameElement( "forenames".to_string() ) )
	}

	/// Returns the full surname including all predicates. Bsp. "von Würzinger".
	pub fn surname_full( &self ) -> Option<String> {
		let res = match &self.predicate {
			Some( x ) => format!( "{} {}", x, &self.surname.as_ref()? ),
			None => self.surname.clone()?,
		};

		Some( res )
	}

	/// Returns the full surname including all predicates. Bsp. "von Würzinger".
	fn surname_full_res( &self ) -> Result<String, NameError> {
		let surname = self.surname.as_ref().ok_or( NameError::MissingNameElement( "surname".to_string() ) )?;
		let res = match &self.predicate {
			Some( x ) => format!( "{} {}", x, surname ),
			None => surname.clone(),
		};

		Ok( res )
	}

	/// This method returns how a persone with the name elements in `self` can be called according to the chose `form` in a specific language (`locale`). If `self` cannot be expressed with `form` (maybe a relevant name part is missing), this method returns an error.
	///
	/// # Arguments
	/// * `form` The name combination.
	/// * `case` the grammatical case.
	/// * `locale` the locale to use the grammatical rules of. Currently only English and German are supported.
	///
	/// # Returns
	/// Returns the calling of the name.
	pub fn designate( &self, form: NameCombo, case: GrammaticalCase, locale: &LanguageIdentifier ) -> Result<String, NameError> {
		match form {
			NameCombo::Name => {
				if self.forenames.is_empty() {
					return Err( NameError::MissingNameElement( "forenames".to_string() ) );
				}
				let res = add_case_letter(
					&format!( "{} {}", self.forenames[0], self.surname_full_res()? ),
					case,
					locale
				)?;
				Ok( res )
			},
			NameCombo::Surname => add_case_letter(
				&self.surname_full_res()?,
				case,
				locale
			),
			NameCombo::Firstname => add_case_letter(
				self.firstname_res()?,
				case,
				locale
			),
			NameCombo::Forenames => add_case_letter(
				&self.forenames_string()?,
				case,
				locale
			),
			NameCombo::Fullname => {
				let name = add_case_letter(
					&format!( "{} {}", self.forenames_string()?, self.surname_full_res()? ),
					case,
					locale
				)?;
				let res = match &self.birthname {
					Some( x ) => format!( "{} geb. {}", name, x ),
					None => name,
				};
				Ok( res )
			},
			NameCombo::Title => self.title.clone().ok_or( NameError::MissingNameElement( "title".to_string() ) ),
			NameCombo::TitleName => {
				let title = self.title.as_ref().ok_or( NameError::MissingNameElement( "title".to_string() ) )?;
				let name = self.designate( NameCombo::Name, case, locale )?;
				Ok( format!( "{} {}", title, name ) )
			},
			NameCombo::TitleFirstname => {
				let title = self.title.as_ref().ok_or( NameError::MissingNameElement( "title".to_string() ) )?;
				let name = self.designate( NameCombo::Firstname, case, locale )?;
				Ok( format!( "{} {}", title, name ) )
			},
			NameCombo::TitleSurname => {
				let title = self.title.as_ref().ok_or( NameError::MissingNameElement( "title".to_string() ) )?;
				Ok( format!( "{} {}", title, self.designate( NameCombo::Surname, case, locale ).unwrap() ) )
			},
			NameCombo::TitleFullname => {
				let title = self.title.as_ref().ok_or( NameError::MissingNameElement( "title".to_string() ) )?;
				let name = self.designate( NameCombo::Fullname, case, locale )?;
				Ok( format!( "{} {}", title, name ) )
			},
			NameCombo::Polite => self.gender
				.ok_or( NameError::MissingNameElement( "gender".to_string() ) )?
				.polite( locale ),
			NameCombo::PoliteName => {
				let polite = self.gender
					.ok_or( NameError::MissingNameElement( "gender".to_string() ) )?
					.polite( locale )?;
				let name = self.designate( NameCombo::Name, case, locale )?;
				Ok( format!( "{} {}", polite, name ) )
			},
			NameCombo::PoliteFirstname => {
				let polite = self.gender
					.ok_or( NameError::MissingNameElement( "gender".to_string() ) )?
					.polite( locale )?;
				let name = self.designate( NameCombo::Firstname, case, locale )?;
				Ok( format!( "{} {}", polite, name ) )
			},
			NameCombo::PoliteSurname => {
				let polite = self.gender
					.ok_or( NameError::MissingNameElement( "gender".to_string() ) )?
					.polite( locale )?;
				Ok( format!( "{} {}", polite, self.designate( NameCombo::Surname, case, locale ).unwrap() ) )
			},
			NameCombo::PoliteFullname => {
				let polite = self.gender
					.ok_or( NameError::MissingNameElement( "gender".to_string() ) )?
					.polite( locale )?;
				let name = self.designate( NameCombo::Fullname, case, locale )?;
				Ok( format!( "{} {}", polite, name ) )
			},
			NameCombo::PoliteTitleName => {
				let polite = self.gender
					.ok_or( NameError::MissingNameElement( "gender".to_string() ) )?
					.polite( locale )?;
				let title = self.title.as_ref()
					.ok_or( NameError::MissingNameElement( "title".to_string() ) )?;
				let name = self.designate( NameCombo::Name, case, locale )?;
				Ok( format!( "{} {} {}", polite, title, name ) )
			},
			NameCombo::Rank => self.rank.clone()
				.ok_or( NameError::MissingNameElement( "title".to_string() ) ),
			NameCombo::RankName => {
				let rank = self.rank.as_ref().ok_or( NameError::MissingNameElement( "rank".to_string() ) )?;
				let name = self.designate( NameCombo::Name, case, locale )?;
				Ok( format!( "{} {}", rank, name ) )
			},
			NameCombo::PoliteRank => {
				let polite = self.gender
					.ok_or( NameError::MissingNameElement( "gender".to_string() ) )?
					.polite( locale )?;
				let rank = self.rank.as_ref().ok_or( NameError::MissingNameElement( "rank".to_string() ) )?;
				Ok( format!( "{} {}", polite, rank ) )
			},
			NameCombo::RankFirstname => {
				let rank = self.rank.as_ref().ok_or( NameError::MissingNameElement( "rank".to_string() ) )?;
				let name = self.designate( NameCombo::Firstname, case, locale )?;
				Ok( format!( "{} {}", rank, name ) )
			},
			NameCombo::RankSurname => {
				let rank = self.rank.as_ref().ok_or( NameError::MissingNameElement( "rank".to_string() ) )?;
				Ok( format!( "{} {}", rank, self.designate( NameCombo::Surname, case, locale ).unwrap() ) )
			},
			NameCombo::RankFullname => {
				let rank = self.rank.as_ref().ok_or( NameError::MissingNameElement( "rank".to_string() ) )?;
				let name = self.designate( NameCombo::Fullname, case, locale )?;
				Ok( format!( "{} {}", rank, name ) )
			},
			NameCombo::RankTitleName => {
				let rank = self.rank.as_ref().ok_or( NameError::MissingNameElement( "rank".to_string() ) )?;
				let title = self.title.as_ref().ok_or( NameError::MissingNameElement( "title".to_string() ) )?;
				let name = self.designate( NameCombo::Name, case, locale )?;
				Ok( format!( "{} {} {}", rank, title, name ) )
			},
			NameCombo::Nickname => add_case_letter(
				self.nickname.as_ref().ok_or( NameError::MissingNameElement( "nickname".to_string() ) )?,
				case,
				locale
			),
			NameCombo::FirstNickname => {
				let name = self.designate( NameCombo::Firstname, case, locale )?;
				let nick = self.nickname.as_ref().ok_or( NameError::MissingNameElement( "nickname".to_string() ) )?;
				Ok( format!( "{} {}", name, nick ) )
			},
			NameCombo::NickSurname => {
				let nick = self.nickname.as_ref().ok_or( NameError::MissingNameElement( "nickname".to_string() ) )?;
				Ok( format!( "{} {}", nick, self.designate( NameCombo::Surname, case, locale )? ) )
			},
			NameCombo::DuaNomina => {
				let nick = self.nickname.as_ref().ok_or( NameError::MissingNameElement( "nickname".to_string() ) )?;
				let surname = self.surname.as_ref().ok_or( NameError::MissingNameElement( "surname".to_string() ) )?;
				add_case_letter( &format!( "{} {}", surname, nick ), case, locale )
			},
			NameCombo::TriaNomina => {
				let name = self.designate( NameCombo::Firstname, case, locale )?;
				let nick = self.nickname.as_ref().ok_or( NameError::MissingNameElement( "nickname".to_string() ) )?;
				let surname = self.surname.as_ref().ok_or( NameError::MissingNameElement( "surname".to_string() ) )?;
				add_case_letter( &format!( "{} {} {}", name, surname, nick ), case, locale )
			},
			NameCombo::Honor => add_case_letter(
				self.honorname.as_ref().ok_or( NameError::MissingNameElement( "honorname".to_string() ) )?,
				case,
				locale
			),
			NameCombo::Honortitle => {
				let honor = self.designate( NameCombo::Honor, case, locale )?;
				let res = match self.gender {
					Some( Gender::Female ) => format!( "Die {}", honor ),
					Some( Gender::Male ) => format!( "Der {}", honor ),
					Some( Gender::Neutral ) => format!( "Das {}", honor ),
					_ => honor.to_string(),
				};
				Ok( res )
			},
			NameCombo::FirstHonorname => {
				let name = self.designate( NameCombo::Firstname, case, locale )?;
				let honor = self.designate( NameCombo::Honor, case, locale )?;
				let res = match self.gender {
					Some( Gender::Female ) => format!( "{} die {}", name, honor ),
					Some( Gender::Male ) => format!( "{} der {}", name, honor ),
					Some( Gender::Neutral ) => format!( "{} das {}", name, honor ),
					_ => format!( "{} {}", name, honor ),
				};
				Ok( res )
			},
			NameCombo::OrderedName => {
				let names = [
					self.firstname(),
					self.predicate.as_deref(),
				];
				let res = format!( "{}, {}",
					self.surname.as_ref().ok_or( NameError::MissingNameElement( "surname".to_string() ) )?,
					names.iter()
						.filter_map( |&x| x )
						.collect::<Vec<&str>>()
						.join( " " )
				);
				add_case_letter( &res, case, locale )
			},
			NameCombo::OrderedSurname => {
				let surname = self.surname.as_ref().ok_or( NameError::MissingNameElement( "surname".to_string() ) )?;
				let res = match &self.predicate {
					Some( x ) => format!( "{}, {}", surname, x ),
					None => surname.clone(),
				};
				add_case_letter( &res, case, locale )
			},
			NameCombo::OrderedTitleName => {
				// let firstname = self.firstname();
				let surname = self.surname.as_ref().ok_or( NameError::MissingNameElement( "surname".to_string() ) )?;
				let names = [
					self.title.as_deref(),
					self.firstname(),
					self.predicate.as_deref(),
				];
				let res = format!( "{}, {}",
					surname,
					names.iter()
						.filter_map( |&x| x )
						.collect::<Vec<&str>>()
						.join( " " )
				);
				add_case_letter( &res, case, locale )
			},
			NameCombo::Initials => {
				let name = self.designate( NameCombo::Name, GrammaticalCase::Nominative, locale )?;
				Ok( initials( &name ) )
			},
			NameCombo::InitialsFull => {
				let forenames = self.designate( NameCombo::Forenames, GrammaticalCase::Nominative, locale )?;
				let mut name_initials = initials( &format!( "{} {}", forenames, self.surname_full_res()? ) );
				if let Some( title ) = &self.title {
					name_initials.insert_str( 0, &format!( "{} ", title ) );
				};
				Ok( name_initials )
			},
			NameCombo::Sign => {
				let forenames = self.designate( NameCombo::Forenames, GrammaticalCase::Nominative, locale )?;
				let name = match &self.predicate {
					Some( x ) => format!( "{} {}", forenames, x ),
					None => forenames,
				};
				let mut name_initials = initials( &name );
				name_initials.push_str(
					&format!( " {}", self.surname.as_ref().ok_or( NameError::MissingNameElement( "surname".to_string() ) )? )
				);
				if let Some( title ) = &self.title {
					name_initials.insert_str( 0, &format!( "{} ", title ) );
				};
				Ok( name_initials )
			},
			NameCombo::Supername => add_case_letter(
				self.supername.as_ref().ok_or( NameError::MissingNameElement( "supername".to_string() ) )?
				, case,
				locale
			),
			NameCombo::FirstSupername => {
				let firstname = self.firstname_res()?;
				let supername = self.designate( NameCombo::Supername, case, locale )?;
				Ok( format!( "{} {}", firstname, supername ) )
			},
			NameCombo::SuperName => {
				let supername = self.designate( NameCombo::Supername, case, locale )?;
				add_case_letter(
					&format!( "{} {} {}", self.firstname_res()?, supername, self.surname_full_res()? ),
					case,
					locale
				)
			},
			NameCombo::PoliteSupername => {
				let polite = self.gender
					.ok_or( NameError::MissingNameElement( "gender".to_string() ) )?
					.polite( locale )?;
				let name = self.designate( NameCombo::Supername, case, locale )?;
				Ok( format!( "{} {}", polite, name ) )
			},
			NameCombo::RankSupername => {
				let rank = self.rank.as_ref().ok_or( NameError::MissingNameElement( "rank".to_string() ) )?;
				let name = self.designate( NameCombo::Supername, case, locale )?;
				Ok( format!( "{} {}", rank, name ) )
			},
		}
	}

	/// Returns a designation by following the following list of precedence, returning the first that is possible. If none of the provided alternatives is available, this function returns `None`.
	///
	/// 1. `NameCombo::Fullname`
	/// 2. `NameCombo::Firstname`
	/// 3. `NameCombo::Surname`
	/// 4. `NameCombo::Nickname`
	/// 5. `NameCombo::Supername`
	///
	/// If the first choice is not available, the next item is tried and so forth until one option is available or none are, in which case this function returns `None`.
	///
	/// # Arguments
	/// * `case` The grammatical case the name will be transformed into.
	pub fn moniker(
		&self,
		case: GrammaticalCase,
		locale: &LanguageIdentifier
	) -> Result<String, NameError> {
		self.designate( NameCombo::Fullname, case, locale )
			.or( self.designate( NameCombo::Firstname, case, locale )
				.or( self.designate( NameCombo::Surname, case, locale )
					.or( self.designate( NameCombo::Nickname, case, locale )
						.or( self.designate( NameCombo::Supername, case, locale ) )
					)
				)
			)
	}
}




//=============================================================================
// Testing


#[cfg( test )]
mod tests {
	use super::*;

	#[test]
	fn grammatical_case_from_str() {
		assert_eq!( GrammaticalCase::from_str( "nominative" ).unwrap(), GrammaticalCase::Nominative );
		assert_eq!( GrammaticalCase::from_str( "Dative" ).unwrap(), GrammaticalCase::Dative );
	}

	#[test]
	fn test_add_case_letter() {
		use unic_langid::LanguageIdentifier;
		use unic_langid::langid;

		const US_ENGLISH: LanguageIdentifier = langid!( "en-US" );
		const GERMAN: LanguageIdentifier = langid!( "de-DE" );

		assert_eq!(
			add_case_letter( "Gunther", GrammaticalCase::Nominative, &US_ENGLISH ).unwrap(),
			"Gunther"
		);
		assert_eq!(
			add_case_letter( "Aristoteles", GrammaticalCase::Nominative, &US_ENGLISH ).unwrap(),
			"Aristoteles"
		);
		assert_eq!(
			add_case_letter( "Günther", GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Günther"
		);
		assert_eq!(
			add_case_letter( "Aristoteles", GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Aristoteles"
		);
		assert_eq!(
			add_case_letter( "Gunther", GrammaticalCase::Genetive, &US_ENGLISH ).unwrap(),
			"Gunther's"
		);
		assert_eq!(
			add_case_letter( "Aristoteles", GrammaticalCase::Genetive, &US_ENGLISH ).unwrap(),
			"Aristoteles'"
		);
		assert_eq!(
			add_case_letter( "Günther", GrammaticalCase::Genetive, &GERMAN ).unwrap(),
			"Günthers"
		);
		assert_eq!(
			add_case_letter( "Aristoteles", GrammaticalCase::Genetive, &GERMAN ).unwrap(),
			"Aristoteles'"
		);
		assert_eq!(
			add_case_letter( "Gunther", GrammaticalCase::Dative, &US_ENGLISH ).unwrap(),
			"Gunther"
		);
		assert_eq!(
			add_case_letter( "Aristoteles", GrammaticalCase::Dative, &US_ENGLISH ).unwrap(),
			"Aristoteles"
		);
		assert_eq!(
			add_case_letter( "Günther", GrammaticalCase::Dative, &GERMAN ).unwrap(),
			"Günther"
		);
		assert_eq!(
			add_case_letter( "Aristoteles", GrammaticalCase::Dative, &GERMAN ).unwrap(),
			"Aristoteles"
		);
		assert_eq!(
			add_case_letter( "Gunther", GrammaticalCase::Accusative, &US_ENGLISH ).unwrap(),
			"Gunther"
		);
		assert_eq!(
			add_case_letter( "Aristoteles", GrammaticalCase::Accusative, &US_ENGLISH ).unwrap(),
			"Aristoteles"
		);
		assert_eq!(
			add_case_letter( "Günther", GrammaticalCase::Accusative, &GERMAN ).unwrap(),
			"Günther"
		);
		assert_eq!(
			add_case_letter( "Aristoteles", GrammaticalCase::Accusative, &GERMAN ).unwrap(),
			"Aristoteles"
		);
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

	#[test]
	fn name_combo_from_str() {
		assert_eq!( NameCombo::from_str( "Name" ).unwrap(), NameCombo::Name );
		assert_eq!( NameCombo::from_str( "PoliteTitleName" ).unwrap(), NameCombo::PoliteTitleName );
	}

	#[test]
	fn test_initials() {
		assert_eq!( initials( "Test Test" ), "T. T.".to_string() );
		assert_eq!( initials( "Thomas von Würzinger" ), "T. v. W.".to_string() );
	}

	#[test]
	fn create_names() {
		assert_eq!( Names::new(), Names::default() );
		assert_eq!( Names::new()
			.with_forenames( &vec![ "Test1", "Test2" ] ), Names {
				forenames: vec![ "Test1".to_string(), "Test2".to_string() ],
				..Default::default()
			}
		);
		assert_eq!( Names::new()
			.with_predicate( "Test" ), Names {
				predicate: Some( "Test".to_string() ),
				..Default::default()
			}
		);
		assert_eq!( Names::new()
			.with_surname( "Test" ), Names {
				surname: Some( "Test".to_string() ),
				..Default::default()
			}
		);
		assert_eq!( Names::new()
			.with_birthname( "Test" ), Names {
				birthname: Some( "Test".to_string() ),
				..Default::default()
			}
		);
		assert_eq!( Names::new()
			.with_title( "Test" ), Names {
				title: Some( "Test".to_string() ),
				..Default::default()
			}
		);
		assert_eq!( Names::new()
			.with_rank( "Test" ), Names {
				rank: Some( "Test".to_string() ),
				..Default::default()
			}
		);
		assert_eq!( Names::new()
			.with_nickname( "Test" ), Names {
				nickname: Some( "Test".to_string() ),
				..Default::default()
			}
		);
		assert_eq!( Names::new()
			.with_honorname( "Test" ), Names {
				honorname: Some( "Test".to_string() ),
				..Default::default()
			}
		);
		assert_eq!( Names::new()
			.with_supername( "Test" ), Names {
				supername: Some( "Test".to_string() ),
				..Default::default()
			}
		);
		assert_eq!( Names::new()
			.with_gender( &Gender::Female ), Names {
				gender: Some( Gender::Female ),
				..Default::default()
			}
		);
	}

	#[test]
	fn name_strings_male() {
		use unic_langid::langid;

		const GERMAN: LanguageIdentifier = langid!( "de-DE" );

		// Thomas Jakob von Würzinger
		let name = Names {
			forenames: [ "Thomas", "Jakob" ].iter().map( |x| x.to_string() ).collect(),
			predicate: Some( "von".to_string() ),
			surname: Some( "Würzinger".to_string() ),
			birthname: None,
			title: None,
			rank: Some( "Hauptkommissar".to_string() ),
			nickname: Some( "Würzi".to_string() ),
			honorname: Some( "Dunkle".to_string() ),
			supername: Some( "Würzt-das-Essen".to_string() ),
			gender: Some( Gender::Male ),
		};

		assert_eq!(
			name.designate( NameCombo::Name, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Thomas von Würzinger".to_string()
		);
		assert_eq!(
			name.designate( NameCombo::Name, GrammaticalCase::Genetive, &GERMAN ).unwrap(),
			"Thomas von Würzingers".to_string()
		);
		assert_eq!(
			name.designate( NameCombo::Name, GrammaticalCase::Accusative, &GERMAN ).unwrap(),
			"Thomas von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Surname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Firstname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Thomas".to_string()
		);
		assert_eq!(
			name.designate( NameCombo::Firstname, GrammaticalCase::Genetive, &GERMAN ).unwrap(),
			"Thomas'".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Forenames, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Thomas Jakob".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Fullname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Thomas Jakob von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Title, GrammaticalCase::Nominative, &GERMAN ),
			Err( NameError::MissingNameElement( "title".to_string() ) )
		);

		assert_eq!(
			name.designate( NameCombo::Polite, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Herr".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteName, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Herr Thomas von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteFirstname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Herr Thomas".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteSurname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Herr von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteFullname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Herr Thomas Jakob von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Nickname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Würzi".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::FirstNickname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Thomas Würzi".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::NickSurname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Würzi von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Supername, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Würzt-das-Essen".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::FirstSupername, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Thomas Würzt-das-Essen".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::SuperName, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Thomas Würzt-das-Essen von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteSupername, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Herr Würzt-das-Essen".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::RankSupername, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Hauptkommissar Würzt-das-Essen".to_string()
		);
	}

	#[test]
	fn name_strings_female() {
		use unic_langid::langid;

		const GERMAN: LanguageIdentifier = langid!( "de-DE" );

		// Penelope Karin von Würzinger geb. Stauff
		let name = Names {
			forenames: [ "Penelope", "Karin" ].iter().map( |x| x.to_string() ).collect(),
			predicate: Some( "von".to_string() ),
			surname: Some( "Würzinger".to_string() ),
			birthname: Some( "Stauff".to_string() ),
			title: Some( "Dr.".to_string() ),
			rank: Some( "Majorin".to_string() ),
			nickname: None,
			honorname: Some( "Große".to_string() ),
			supername: None,
			gender: Some( Gender::Female ),
		};

		assert_eq!(
			name.designate( NameCombo::Name, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Penelope von Würzinger".to_string()
		);
		assert_eq!(
			name.designate( NameCombo::Name, GrammaticalCase::Genetive, &GERMAN ).unwrap(),
			"Penelope von Würzingers".to_string()
		);
		assert_eq!(
			name.designate( NameCombo::Name, GrammaticalCase::Accusative, &GERMAN ).unwrap(),
			"Penelope von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Surname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Firstname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Penelope".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Fullname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Penelope Karin von Würzinger geb. Stauff".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Title, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Dr.".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::TitleName, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Dr. Penelope von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::TitleFirstname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Dr. Penelope".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::TitleSurname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Dr. von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::TitleFullname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Dr. Penelope Karin von Würzinger geb. Stauff".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Polite, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Frau".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteName, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Frau Penelope von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteFirstname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Frau Penelope".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteSurname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Frau von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteFullname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Frau Penelope Karin von Würzinger geb. Stauff".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteTitleName, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Frau Dr. Penelope von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Rank, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Majorin".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteRank, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Frau Majorin".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::RankName, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Majorin Penelope von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::RankFirstname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Majorin Penelope".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::RankSurname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Majorin von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::RankFullname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Majorin Penelope Karin von Würzinger geb. Stauff".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::RankTitleName, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Majorin Dr. Penelope von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Honor, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Große".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Honortitle, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Die Große".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::FirstHonorname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Penelope die Große".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::OrderedName, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Würzinger, Penelope von".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::OrderedSurname, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Würzinger, von".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::OrderedTitleName, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Würzinger, Dr. Penelope von".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Initials, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"P. v. W.".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::InitialsFull, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Dr. P. K. v. W.".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Sign, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Dr. P. K. v. Würzinger".to_string()
		);
	}

	#[test]
	fn name_strings_roman_male() {
		use unic_langid::langid;

		const GERMAN: LanguageIdentifier = langid!( "de-DE" );

		// Gaius Julius Caesar
		let name = Names {
			forenames: vec![ "Gaius".to_string() ],
			predicate: None,
			surname: Some( "Julius".to_string() ),
			birthname: None,
			title: None,
			rank: None,
			nickname: Some( "Caesar".to_string() ),
			honorname: None,
			supername: None,
			gender: None,
		};

		assert_eq!(
			name.designate( NameCombo::TriaNomina, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Gaius Julius Caesar".to_string()
		);
	}

	#[test]
	fn name_strings_roman_female() {
		use unic_langid::langid;

		const GERMAN: LanguageIdentifier = langid!( "de-DE" );

		// Iunia Prima
		let name = Names {
			forenames: Vec::new(),
			predicate: None,
			surname: Some( "Iunia".to_string() ),
			birthname: None,
			title: None,
			rank: None,
			nickname: Some( "Prima".to_string() ),
			honorname: None,
			supername: None,
			gender: None,
		};

		assert_eq!(
			name.designate( NameCombo::DuaNomina, GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Iunia Prima".to_string()
		);
	}

	#[test]
	fn name_moniker() {
		use unic_langid::langid;

		const GERMAN: LanguageIdentifier = langid!( "de-DE" );

		assert_eq!( Names::new().moniker( GrammaticalCase::Nominative, &GERMAN ), Err( NameError::MissingNameElement( "supername".to_string() ) ) );
		assert_eq!(
			Names::new()
				.with_forenames( &[ "Penelope", "Karin" ] )
				.moniker( GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Penelope".to_string()
		);
		assert_eq!(
			Names::new()
				.with_forenames( &[ "Penelope", "Karin" ] )
				.with_surname( "Würzinger" )
				.moniker( GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Penelope Karin Würzinger".to_string()
		);
		assert_eq!(
			Names::new()
				.with_forenames( &[ "Penelope", "Karin" ] )
				.with_predicate( "von" )
				.with_surname( "Würzinger" )
				.moniker( GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Penelope Karin von Würzinger".to_string()
		);
		assert_eq!(
			Names::new()
				.with_nickname( "Würzli" )
				.moniker( GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Würzli".to_string()
		);
		assert_eq!(
			Names::new()
				.with_nickname( "Würzli" )
				.with_surname( "Würzinger" )
				.moniker( GrammaticalCase::Nominative, &GERMAN ).unwrap(),
			"Würzinger".to_string()
		);
	}
}
