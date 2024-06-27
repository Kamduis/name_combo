//! This crate provides the means to save and represent person's names.
//!
//! In its current state this crate concentrates on German names but can be used to represent a variety of names of different languages.
//!
//! # Optional Features
//! * **serde** Enables `serde` support.




//=============================================================================
// Crates


use std::hash::Hash;
use std::fmt;
use std::str::FromStr;

#[allow( unused )]
use log::{error, warn, info, debug};

#[cfg( feature = "serde" )]
use serde::{Serialize, Deserialize};

use thiserror::Error;




//=============================================================================
// Errors


#[derive( Error, Debug )]
pub enum NameError {
	#[error( "This grammatical case is illegal." )]
	IllegalCase,

	#[error( "This name combo is illegal." )]
	IllegalCombo,
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


/// Adding letters to `text` depending on the grammatical case. `text` is assumed to be of the Nominative case.
///
/// Bsp. Günther
/// => Günther (Nominative)
/// => Günthers (Genetive)
/// => Günther (Dative)
/// => Günthers (Accusative)
fn add_case_letter( text: &str, case: GrammaticalCase ) -> String {
	if text.is_empty() {
		return "".to_string();
	}

	let glyph_last = text.chars()
		.last().unwrap()
		.to_lowercase()
		.to_string();

	let appendix = match case {
		GrammaticalCase::Nominative | GrammaticalCase::Dative => "",
		GrammaticalCase::Genetive => match glyph_last.as_str() {
			"s" | "z" => "'",
			_ => "s",
		},
		GrammaticalCase::Accusative => match glyph_last.as_str() {
			"n" => "n",
			_ => "",
		},
	};

	format!( "{}{}", text, appendix )
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
	/// Returns the german polite adress for a person of the respective gender.
	fn polite( &self ) -> Option<String> {
		match self {
			Self::Male    => Some( "Herr".to_string() ),
			Self::Female  => Some( "Frau".to_string() ),
			Self::Neutral | Self::Other => None,
		}
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
			Self::Male    => "männlich",
			Self::Female  => "weiblich",
			Self::Neutral => "neutral",
			Self::Other   => "anders",
		};

		write!( f, "{}", res )
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

	/// Returns all fornames.
	pub fn forenames( &self ) -> &Vec<String> {
		&self.forenames
	}

	/// Returns all fornames as a string. Bsp. "Thomas Jakob". If no forename is given, this returns `None`.
	fn forenames_string( &self ) -> Option<String> {
		if self.forenames.is_empty() {
			return None;
		}
		Some( self.forenames.join( " " ) )
	}

	/// Returns the first forname. If no forenames are given, this method returns `None`.
	pub fn firstname( &self ) -> Option<&String> {
		self.forenames.first()
	}

	/// Returns the full surname including all predicates. Bsp. "von Würzinger".
	pub fn surname_full( &self ) -> Option<String> {
		let res = match &self.predicate {
			Some( x ) => format!( "{} {}", x, &self.surname.clone()? ),
			None => self.surname.clone()?,
		};

		Some( res )
	}

	/// Returns a calling of a name.
	pub fn designate( &self, form: NameCombo, case: GrammaticalCase ) -> Option<String> {
		match form {
			NameCombo::Name => {
				if self.forenames.is_empty() {
					return None
				}
				let res = add_case_letter( &format!( "{} {}", self.forenames[0], self.surname_full()? ), case );
				Some( res )
			},
			NameCombo::Surname => Some( add_case_letter( &self.surname_full()?, case ) ),
			NameCombo::Firstname => self.firstname().as_ref().map( |x| add_case_letter( x, case ) ),
			NameCombo::Forenames => self.forenames_string().map( |x| add_case_letter( &x, case ) ),
			NameCombo::Fullname => {
				if self.forenames.is_empty() {
					return None
				}
				let name = add_case_letter( &format!( "{} {}", self.forenames_string().unwrap(), self.surname_full()? ), case );
				let res = match &self.birthname {
					Some( x ) => format!( "{} geb. {}", name, x ),
					None => name,
				};
				Some( res )
			},
			NameCombo::Title => self.title.clone(),
			NameCombo::TitleName => {
				let title = self.title.as_ref()?;
				let name = self.designate( NameCombo::Name, case )?;
				Some( format!( "{} {}", title, name ) )
			},
			NameCombo::TitleFirstname => {
				let title = self.title.as_ref()?;
				let name = self.designate( NameCombo::Firstname, case )?;
				Some( format!( "{} {}", title, name ) )
			},
			NameCombo::TitleSurname => {
				let title = self.title.as_ref()?;
				Some( format!( "{} {}", title, self.designate( NameCombo::Surname, case ).unwrap() ) )
			},
			NameCombo::TitleFullname => {
				let title = self.title.as_ref()?;
				let name = self.designate( NameCombo::Fullname, case )?;
				Some( format!( "{} {}", title, name ) )
			},
			NameCombo::Polite => self.gender?.polite(),
			NameCombo::PoliteName => {
				let polite = self.gender?.polite()?;
				let name = self.designate( NameCombo::Name, case )?;
				Some( format!( "{} {}", polite, name ) )
			},
			NameCombo::PoliteFirstname => {
				let polite = self.gender?.polite()?;
				let name = self.designate( NameCombo::Firstname, case )?;
				Some( format!( "{} {}", polite, name ) )
			},
			NameCombo::PoliteSurname => {
				let polite = self.gender?.polite()?;
				Some( format!( "{} {}", polite, self.designate( NameCombo::Surname, case ).unwrap() ) )
			},
			NameCombo::PoliteFullname => {
				let polite = self.gender?.polite()?;
				let name = self.designate( NameCombo::Fullname, case )?;
				Some( format!( "{} {}", polite, name ) )
			},
			NameCombo::PoliteTitleName => {
				let polite = self.gender?.polite()?;
				let title = self.title.as_ref()?;
				let name = self.designate( NameCombo::Name, case )?;
				Some( format!( "{} {} {}", polite, title, name ) )
			},
			NameCombo::Rank => self.rank.clone(),
			NameCombo::RankName => {
				let rank = self.rank.as_ref()?;
				let name = self.designate( NameCombo::Name, case )?;
				Some( format!( "{} {}", rank, name ) )
			},
			NameCombo::PoliteRank => {
				let polite = self.gender?.polite()?;
				let rank = self.rank.as_ref()?;
				Some( format!( "{} {}", polite, rank ) )
			},
			NameCombo::RankFirstname => {
				let rank = self.rank.as_ref()?;
				let name = self.designate( NameCombo::Firstname, case )?;
				Some( format!( "{} {}", rank, name ) )
			},
			NameCombo::RankSurname => {
				let rank = self.rank.as_ref()?;
				Some( format!( "{} {}", rank, self.designate( NameCombo::Surname, case ).unwrap() ) )
			},
			NameCombo::RankFullname => {
				let rank = self.rank.as_ref()?;
				let name = self.designate( NameCombo::Fullname, case )?;
				Some( format!( "{} {}", rank, name ) )
			},
			NameCombo::RankTitleName => {
				let rank = self.rank.as_ref()?;
				let title = self.title.as_ref()?;
				let name = self.designate( NameCombo::Name, case )?;
				Some( format!( "{} {} {}", rank, title, name ) )
			},
			NameCombo::Nickname => self.nickname.as_ref().map( |x| add_case_letter( x, case ) ),
			NameCombo::FirstNickname => {
				let name = self.designate( NameCombo::Firstname, case )?;
				let nick = self.nickname.as_ref()?;
				Some( format!( "{} {}", name, nick ) )
			},
			NameCombo::NickSurname => {
				let nick = self.nickname.as_ref()?;
				Some( format!( "{} {}", nick, self.designate( NameCombo::Surname, case ).unwrap() ) )
			},
			NameCombo::DuaNomina => {
				let nick = self.nickname.as_ref()?;
				let res = add_case_letter( &format!( "{} {}", self.surname.clone()?, nick ), case );
				Some( res )
			},
			NameCombo::TriaNomina => {
				let name = self.designate( NameCombo::Firstname, case )?;
				let nick = self.nickname.as_ref()?;
				let res = add_case_letter( &format!( "{} {} {}", name, self.surname.clone()?, nick ), case );
				Some( res )
			},
			NameCombo::Honor => self.honorname.as_ref().map( |x| add_case_letter( x, case ) ),
			NameCombo::Honortitle => {
				let honor = self.designate( NameCombo::Honor, case )?;
				let res = match self.gender {
					Some( Gender::Female ) => format!( "Die {}", honor ),
					Some( Gender::Male ) => format!( "Der {}", honor ),
					Some( Gender::Neutral ) => format!( "Das {}", honor ),
					_ => honor.to_string(),
				};
				Some( res )
			},
			NameCombo::FirstHonorname => {
				let name = self.designate( NameCombo::Firstname, case )?;
				let honor = self.designate( NameCombo::Honor, case )?;
				let res = match self.gender {
					Some( Gender::Female ) => format!( "{} die {}", name, honor ),
					Some( Gender::Male ) => format!( "{} der {}", name, honor ),
					Some( Gender::Neutral ) => format!( "{} das {}", name, honor ),
					_ => format!( "{} {}", name, honor ),
				};
				Some( res )
			},
			NameCombo::OrderedName => {
				let names = [
					self.firstname(),
					self.predicate.as_ref(),
				];
				let res = format!( "{}, {}",
					self.surname.clone()?,
					names.iter()
						.filter_map( |&x| x.cloned() )
						.collect::<Vec<String>>()
						.join( " " )
				);
				Some( add_case_letter( &res, case ) )
			},
			NameCombo::OrderedSurname => {
				let res = match &self.predicate {
					Some( x ) => format!( "{}, {}", self.surname.clone()?, x ),
					None => self.surname.clone()?,
				};
				Some( add_case_letter( &res, case ) )
			},
			NameCombo::OrderedTitleName => {
				let firstname = self.firstname().cloned();
				let names = [
					&self.title,
					&firstname,
					&self.predicate,
				];
				let res = format!( "{}, {}",
					self.surname.clone()?,
					names.iter()
						.filter_map( |&x| x.clone() )
						.collect::<Vec<String>>()
						.join( " " )
				);
				Some( add_case_letter( &res, case ) )
			},
			NameCombo::Initials => {
				let name = self.designate( NameCombo::Name, GrammaticalCase::Nominative )?;
				Some( initials( &name ) )
			},
			NameCombo::InitialsFull => {
				let forenames = self.designate( NameCombo::Forenames, GrammaticalCase::Nominative )?;
				let mut name_initials = initials( &format!( "{} {}", forenames, self.surname_full()? ) );
				if let Some( title ) = &self.title {
					name_initials.insert_str( 0, &format!( "{} ", title ) );
				};
				Some( name_initials )
			},
			NameCombo::Sign => {
				let forenames = self.designate( NameCombo::Forenames, GrammaticalCase::Nominative )?;
				let name = match &self.predicate {
					Some( x ) => format!( "{} {}", forenames, x ),
					None => forenames,
				};
				let mut name_initials = initials( &name );
				name_initials.push_str( &format!( " {}", self.surname.clone()? ) );
				if let Some( title ) = &self.title {
					name_initials.insert_str( 0, &format!( "{} ", title ) );
				};
				Some( name_initials )
			},
			NameCombo::Supername => self.supername.as_ref().map( |x| add_case_letter( x, case ) ),
			NameCombo::FirstSupername => {
				let firstname = self.firstname()?;
				let supername = self.designate( NameCombo::Supername, case )?;
				Some( format!( "{} {}", firstname, supername ) )
			},
			NameCombo::SuperName => {
				if self.forenames.is_empty() {
					return None
				}
				let supername = self.designate( NameCombo::Supername, case )?;
				let res = add_case_letter( &format!( "{} {} {}", self.forenames[0], supername, self.surname_full()? ), case );
				Some( res )
			},
			NameCombo::PoliteSupername => {
				let polite = self.gender?.polite()?;
				let name = self.designate( NameCombo::Supername, case )?;
				Some( format!( "{} {}", polite, name ) )
			},
			NameCombo::RankSupername => {
				let rank = self.rank.as_ref()?;
				let name = self.designate( NameCombo::Supername, case )?;
				Some( format!( "{} {}", rank, name ) )
			},
		}
	}

	/// Returns a designation by following the following list of precidence, returning the first that is possible. If none of the provided alternatives is available, this function returns `None`.
	///
	/// 1. `NameCombo::Fullname`
	/// 2. `NameCombo::Firstname`
	/// 3. `NameCombo::Surname`
	/// 4. `NameCombo::Nickname`
	/// 5. `NameCombo::Supername`
	///
	/// If the first choice is not available, the next item is tried and so forth until one option is available or none are, iin which case this function returns `None`.
	///
	/// # Arguments
	/// * `case` The grammatical case the name will be transformed into.
	pub fn moniker(
		&self,
		case: GrammaticalCase
	) -> Option<String> {
		self.designate( NameCombo::Fullname, case )
			.or( self.designate( NameCombo::Firstname, case )
				.or( self.designate( NameCombo::Surname, case )
					.or( self.designate( NameCombo::Nickname, case )
						.or( self.designate( NameCombo::Supername, case ) )
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
	fn gender_title() {
		assert_eq!( Gender::Male.polite().unwrap(), "Herr".to_string() );
		assert_eq!( Gender::Female.polite().unwrap(), "Frau".to_string() );
		assert!( Gender::Neutral.polite().is_none() );
		assert!( Gender::Other.polite().is_none() );
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
		assert_eq!( Gender::Male.to_string(), "männlich".to_string() );
		assert_eq!( Gender::Female.to_string(), "weiblich".to_string() );
		assert_eq!( Gender::Neutral.to_string(), "neutral".to_string() );
		assert_eq!( Gender::Other.to_string(), "anders".to_string() );
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
			name.designate( NameCombo::Name, GrammaticalCase::Nominative ).unwrap(),
			"Thomas von Würzinger".to_string()
		);
		assert_eq!(
			name.designate( NameCombo::Name, GrammaticalCase::Genetive ).unwrap(),
			"Thomas von Würzingers".to_string()
		);
		assert_eq!(
			name.designate( NameCombo::Name, GrammaticalCase::Accusative ).unwrap(),
			"Thomas von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Surname, GrammaticalCase::Nominative ).unwrap(),
			"von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Firstname, GrammaticalCase::Nominative ).unwrap(),
			"Thomas".to_string()
		);
		assert_eq!(
			name.designate( NameCombo::Firstname, GrammaticalCase::Genetive ).unwrap(),
			"Thomas'".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Forenames, GrammaticalCase::Nominative ).unwrap(),
			"Thomas Jakob".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Fullname, GrammaticalCase::Nominative ).unwrap(),
			"Thomas Jakob von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Title, GrammaticalCase::Nominative ),
			None
		);

		assert_eq!(
			name.designate( NameCombo::Polite, GrammaticalCase::Nominative ).unwrap(),
			"Herr".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteName, GrammaticalCase::Nominative ).unwrap(),
			"Herr Thomas von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteFirstname, GrammaticalCase::Nominative ).unwrap(),
			"Herr Thomas".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteSurname, GrammaticalCase::Nominative ).unwrap(),
			"Herr von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteFullname, GrammaticalCase::Nominative ).unwrap(),
			"Herr Thomas Jakob von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Nickname, GrammaticalCase::Nominative ).unwrap(),
			"Würzi".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::FirstNickname, GrammaticalCase::Nominative ).unwrap(),
			"Thomas Würzi".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::NickSurname, GrammaticalCase::Nominative ).unwrap(),
			"Würzi von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Supername, GrammaticalCase::Nominative ).unwrap(),
			"Würzt-das-Essen".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::FirstSupername, GrammaticalCase::Nominative ).unwrap(),
			"Thomas Würzt-das-Essen".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::SuperName, GrammaticalCase::Nominative ).unwrap(),
			"Thomas Würzt-das-Essen von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteSupername, GrammaticalCase::Nominative ).unwrap(),
			"Herr Würzt-das-Essen".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::RankSupername, GrammaticalCase::Nominative ).unwrap(),
			"Hauptkommissar Würzt-das-Essen".to_string()
		);
	}

	#[test]
	fn name_strings_female() {
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
			name.designate( NameCombo::Name, GrammaticalCase::Nominative ).unwrap(),
			"Penelope von Würzinger".to_string()
		);
		assert_eq!(
			name.designate( NameCombo::Name, GrammaticalCase::Genetive ).unwrap(),
			"Penelope von Würzingers".to_string()
		);
		assert_eq!(
			name.designate( NameCombo::Name, GrammaticalCase::Accusative ).unwrap(),
			"Penelope von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Surname, GrammaticalCase::Nominative ).unwrap(),
			"von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Firstname, GrammaticalCase::Nominative ).unwrap(),
			"Penelope".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Fullname, GrammaticalCase::Nominative ).unwrap(),
			"Penelope Karin von Würzinger geb. Stauff".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Title, GrammaticalCase::Nominative ).unwrap(),
			"Dr.".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::TitleName, GrammaticalCase::Nominative ).unwrap(),
			"Dr. Penelope von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::TitleFirstname, GrammaticalCase::Nominative ).unwrap(),
			"Dr. Penelope".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::TitleSurname, GrammaticalCase::Nominative ).unwrap(),
			"Dr. von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::TitleFullname, GrammaticalCase::Nominative ).unwrap(),
			"Dr. Penelope Karin von Würzinger geb. Stauff".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Polite, GrammaticalCase::Nominative ).unwrap(),
			"Frau".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteName, GrammaticalCase::Nominative ).unwrap(),
			"Frau Penelope von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteFirstname, GrammaticalCase::Nominative ).unwrap(),
			"Frau Penelope".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteSurname, GrammaticalCase::Nominative ).unwrap(),
			"Frau von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteFullname, GrammaticalCase::Nominative ).unwrap(),
			"Frau Penelope Karin von Würzinger geb. Stauff".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteTitleName, GrammaticalCase::Nominative ).unwrap(),
			"Frau Dr. Penelope von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Rank, GrammaticalCase::Nominative ).unwrap(),
			"Majorin".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::PoliteRank, GrammaticalCase::Nominative ).unwrap(),
			"Frau Majorin".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::RankName, GrammaticalCase::Nominative ).unwrap(),
			"Majorin Penelope von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::RankFirstname, GrammaticalCase::Nominative ).unwrap(),
			"Majorin Penelope".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::RankSurname, GrammaticalCase::Nominative ).unwrap(),
			"Majorin von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::RankFullname, GrammaticalCase::Nominative ).unwrap(),
			"Majorin Penelope Karin von Würzinger geb. Stauff".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::RankTitleName, GrammaticalCase::Nominative ).unwrap(),
			"Majorin Dr. Penelope von Würzinger".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Honor, GrammaticalCase::Nominative ).unwrap(),
			"Große".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Honortitle, GrammaticalCase::Nominative ).unwrap(),
			"Die Große".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::FirstHonorname, GrammaticalCase::Nominative ).unwrap(),
			"Penelope die Große".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::OrderedName, GrammaticalCase::Nominative ).unwrap(),
			"Würzinger, Penelope von".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::OrderedSurname, GrammaticalCase::Nominative ).unwrap(),
			"Würzinger, von".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::OrderedTitleName, GrammaticalCase::Nominative ).unwrap(),
			"Würzinger, Dr. Penelope von".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Initials, GrammaticalCase::Nominative ).unwrap(),
			"P. v. W.".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::InitialsFull, GrammaticalCase::Nominative ).unwrap(),
			"Dr. P. K. v. W.".to_string()
		);

		assert_eq!(
			name.designate( NameCombo::Sign, GrammaticalCase::Nominative ).unwrap(),
			"Dr. P. K. v. Würzinger".to_string()
		);
	}

	#[test]
	fn name_strings_roman_male() {
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
			name.designate( NameCombo::TriaNomina, GrammaticalCase::Nominative ).unwrap(),
			"Gaius Julius Caesar".to_string()
		);
	}

	#[test]
	fn name_strings_roman_female() {
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
			name.designate( NameCombo::DuaNomina, GrammaticalCase::Nominative ).unwrap(),
			"Iunia Prima".to_string()
		);
	}

	#[test]
	fn name_moniker() {
		assert_eq!( Names::new().moniker( GrammaticalCase::Nominative ), None );
		assert_eq!(
			Names::new()
				.with_forenames( &[ "Penelope", "Karin" ] )
				.moniker( GrammaticalCase::Nominative ).unwrap(),
			"Penelope".to_string()
		);
		assert_eq!(
			Names::new()
				.with_forenames( &[ "Penelope", "Karin" ] )
				.with_surname( "Würzinger" )
				.moniker( GrammaticalCase::Nominative ).unwrap(),
			"Penelope Karin Würzinger".to_string()
		);
		assert_eq!(
			Names::new()
				.with_forenames( &[ "Penelope", "Karin" ] )
				.with_predicate( "von" )
				.with_surname( "Würzinger" )
				.moniker( GrammaticalCase::Nominative ).unwrap(),
			"Penelope Karin von Würzinger".to_string()
		);
		assert_eq!(
			Names::new()
				.with_nickname( "Würzli" )
				.moniker( GrammaticalCase::Nominative ).unwrap(),
			"Würzli".to_string()
		);
		assert_eq!(
			Names::new()
				.with_nickname( "Würzli" )
				.with_surname( "Würzinger" )
				.moniker( GrammaticalCase::Nominative ).unwrap(),
			"Würzinger".to_string()
		);
	}
}
