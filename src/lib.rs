//! This crate provides the means to save and represent person's names.
//!
//! In its current state this crate concentrates on german names but can be used to represent a variety of names of different languages.




//=============================================================================
// Crates


use std::fmt;




//=============================================================================
// Helper functions


/// Creating initials from `text` by only taking the first letter of each word and adding a dot after it.
///
/// Bsp. "Thomas von Würzinger" => "T. v. W."
fn initials( text: &str ) -> String {
	if text.is_empty() {
		return "".to_string();
	}

	text.split( " " )
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
#[derive( Clone, Copy, Debug )]
pub enum GrammaticalCase {
	Nominative,
	Genetive,
	Dative,
	Accusative,
}


/// A subset of possible genders.
#[derive( Clone, Copy, PartialEq, Eq, Debug )]
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
}

impl fmt::Display for Gender {
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		let res = match self {
			Self::Male    => "♂",
			Self::Female  => "♀",
			Self::Neutral => "⚪",
			Self::Other   => "⚧",
		};

		write!( f, "{}", res )
	}
}


/// The possible combination of names.
#[derive( Debug )]
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

	Supername,

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




//=============================================================================
// Structs


/// The different names of a person that can be combined in various ways.
pub struct Names {
	forenames: Vec<String>,
	predicate: Option<String>,
	surname: String,
	birthname: Option<String>,
	title: Option<String>,
	rank: Option<String>,
	nickname: Option<String>,
	honorname: Option<String>,
	supername: Option<String>,
	gender: Gender,
}

impl Names {
	/// Returns all fornames as a string. Bsp. "Thomas Jakob". If no forename is given, this returns `None`.
	fn forenames( &self ) -> Option<String> {
		if self.forenames.is_empty() {
			return None;
		}
		Some( self.forenames.join( " " ) )
	}

	/// Returns the full surname including all predicates. Bsp. "von Würzinger".
	fn surname_full( &self ) -> String {
		match &self.predicate {
			Some( x ) => format!( "{} {}", x, self.surname ),
			None => self.surname.clone(),
		}
	}

	/// Returns a calling of a name.
	pub fn designate( &self, form: NameCombo, case: GrammaticalCase ) -> Option<String> {
		match form {
			NameCombo::Name => {
				if self.forenames.is_empty() {
					return None
				}
				let res = add_case_letter( &format!( "{} {}", self.forenames[0], self.surname_full() ), case );
				Some( res )
			},
			NameCombo::Surname => Some( add_case_letter( &self.surname_full(), case ) ),
			NameCombo::Firstname => self.forenames.first().map( |x| add_case_letter( x, case ) ),
			NameCombo::Forenames => self.forenames().map( |x| add_case_letter( &x, case ) ),
			NameCombo::Fullname => {
				if self.forenames.is_empty() {
					return None
				}
				let name = add_case_letter( &format!( "{} {}", self.forenames().unwrap(), self.surname_full() ), case );
				let res = match &self.birthname {
					Some( x ) => format!( "{} geb. {}", name, x ),
					None => name,
				};
				Some( res )
			},
			NameCombo::Title => self.title.clone(),
			NameCombo::TitleName => {
				let Some( title ) = self.title.clone() else {
					return None;
				};
				let Some( name ) = self.designate( NameCombo::Name, case ) else {
					return None;
				};
				Some( format!( "{} {}", title, name ) )
			},
			NameCombo::TitleFirstname => {
				let Some( title ) = self.title.clone() else {
					return None;
				};
				let Some( name ) = self.designate( NameCombo::Firstname, case ) else {
					return None;
				};
				Some( format!( "{} {}", title, name ) )
			},
			NameCombo::TitleSurname => {
				let Some( title ) = self.title.clone() else {
					return None;
				};
				Some( format!( "{} {}", title, self.designate( NameCombo::Surname, case ).unwrap() ) )
			},
			NameCombo::TitleFullname => {
				let Some( title ) = self.title.clone() else {
					return None;
				};
				let Some( name ) = self.designate( NameCombo::Fullname, case ) else {
					return None;
				};
				Some( format!( "{} {}", title, name ) )
			},
			NameCombo::Polite => self.gender.polite(),
			NameCombo::PoliteName => {
				let Some( polite ) = self.gender.polite() else {
					return None;
				};
				let Some( name ) = self.designate( NameCombo::Name, case ) else {
					return None;
				};
				Some( format!( "{} {}", polite, name ) )
			},
			NameCombo::PoliteFirstname => {
				let Some( polite ) = self.gender.polite() else {
					return None;
				};
				let Some( name ) = self.designate( NameCombo::Firstname, case ) else {
					return None;
				};
				Some( format!( "{} {}", polite, name ) )
			},
			NameCombo::PoliteSurname => {
				let Some( polite ) = self.gender.polite() else {
					return None;
				};
				Some( format!( "{} {}", polite, self.designate( NameCombo::Surname, case ).unwrap() ) )
			},
			NameCombo::PoliteFullname => {
				let Some( polite ) = self.gender.polite() else {
					return None;
				};
				let Some( name ) = self.designate( NameCombo::Fullname, case ) else {
					return None;
				};
				Some( format!( "{} {}", polite, name ) )
			},
			NameCombo::PoliteTitleName => {
				let Some( polite ) = self.gender.polite() else {
					return None;
				};
				let Some( title ) = self.title.clone() else {
					return None;
				};
				let Some( name ) = self.designate( NameCombo::Name, case ) else {
					return None;
				};
				Some( format!( "{} {} {}", polite, title, name ) )
			},
			NameCombo::Rank => self.rank.clone(),
			NameCombo::RankName => {
				let Some( rank ) = self.rank.clone() else {
					return None;
				};
				let Some( name ) = self.designate( NameCombo::Name, case ) else {
					return None;
				};
				Some( format!( "{} {}", rank, name ) )
			},
			NameCombo::PoliteRank => {
				let Some( polite ) = self.gender.polite() else {
					return None;
				};
				let Some( rank ) = self.rank.clone() else {
					return None;
				};
				Some( format!( "{} {}", polite, rank ) )
			},
			NameCombo::RankFirstname => {
				let Some( rank ) = self.rank.clone() else {
					return None;
				};
				let Some( name ) = self.designate( NameCombo::Firstname, case ) else {
					return None;
				};
				Some( format!( "{} {}", rank, name ) )
			},
			NameCombo::RankSurname => {
				let Some( rank ) = self.rank.clone() else {
					return None;
				};
				Some( format!( "{} {}", rank, self.designate( NameCombo::Surname, case ).unwrap() ) )
			},
			NameCombo::RankFullname => {
				let Some( rank ) = self.rank.clone() else {
					return None;
				};
				let Some( name ) = self.designate( NameCombo::Fullname, case ) else {
					return None;
				};
				Some( format!( "{} {}", rank, name ) )
			},
			NameCombo::RankTitleName => {
				let Some( rank ) = self.rank.clone() else {
					return None;
				};
				let Some( title ) = self.title.clone() else {
					return None;
				};
				let Some( name ) = self.designate( NameCombo::Name, case ) else {
					return None;
				};
				Some( format!( "{} {} {}", rank, title, name ) )
			},
			NameCombo::Nickname => self.nickname.as_ref().map( |x| add_case_letter( x, case ) ),
			NameCombo::FirstNickname => {
				let Some( name ) = self.designate( NameCombo::Firstname, case ) else {
					return None;
				};
				let Some( nick ) = self.nickname.clone() else {
					return None;
				};
				Some( format!( "{} {}", name, nick ) )
			},
			NameCombo::NickSurname => {
				let Some( nick ) = self.nickname.clone() else {
					return None;
				};
				Some( format!( "{} {}", nick, self.designate( NameCombo::Surname, case ).unwrap() ) )
			},
			NameCombo::DuaNomina => {
				let Some( nick ) = self.nickname.clone() else {
					return None;
				};
				let res = add_case_letter( &format!( "{} {}", self.surname, nick ), case );
				Some( res )
			},
			NameCombo::TriaNomina => {
				let Some( name ) = self.designate( NameCombo::Firstname, case ) else {
					return None;
				};
				let Some( nick ) = self.nickname.clone() else {
					return None;
				};
				let res = add_case_letter( &format!( "{} {} {}", name, self.surname, nick ), case );
				Some( res )
			},
			NameCombo::Honor => self.honorname.as_ref().map( |x| add_case_letter( x, case ) ),
			NameCombo::Honortitle => {
				let Some( honor ) = self.designate( NameCombo::Honor, case ) else {
					return None;
				};
				let article = if self.gender == Gender::Female {
					"Die"
				} else {
					"Der"
				};
				Some( format!( "{} {}", article, honor ) )
			},
			NameCombo::FirstHonorname => {
				let Some( name ) = self.designate( NameCombo::Firstname, case ) else {
					return None;
				};
				let Some( honor ) = self.designate( NameCombo::Honor, case ) else {
					return None;
				};
				let article = if self.gender == Gender::Female {
					"die"
				} else {
					"der"
				};
				Some( format!( "{} {} {}", name, article, honor ) )
			},
			NameCombo::OrderedName => {
				let firstname = self.forenames.get( 0 ).cloned();
				let names = vec![
					&firstname,
					&self.predicate,
				];
				let res = format!( "{}, {}",
					self.surname,
					names.iter()
						.filter_map( |&x| x.clone() )
						.collect::<Vec<String>>()
						.join( " " )
				);
				Some( add_case_letter( &res, case ) )
			},
			NameCombo::OrderedSurname => {
				let res = match &self.predicate {
					Some( x ) => format!( "{}, {}", self.surname, x ),
					None => self.surname.clone(),
				};
				Some( add_case_letter( &res, case ) )
			},
			NameCombo::OrderedTitleName => {
				let firstname = self.forenames.get( 0 ).cloned();
				let names = vec![
					&self.title,
					&firstname,
					&self.predicate,
				];
				let res = format!( "{}, {}",
					self.surname,
					names.iter()
						.filter_map( |&x| x.clone() )
						.collect::<Vec<String>>()
						.join( " " )
				);
				Some( add_case_letter( &res, case ) )
			},
			NameCombo::Initials => {
				let Some( name ) = self.designate( NameCombo::Name, GrammaticalCase::Nominative ) else {
					return None;
				};
				Some( initials( &name ) )
			},
			NameCombo::InitialsFull => {
				let Some( forenames ) = self.designate( NameCombo::Forenames, GrammaticalCase::Nominative ) else {
					return None;
				};
				let Some( surname ) = self.designate( NameCombo::Surname, GrammaticalCase::Nominative ) else {
					return None;
				};
				let mut name_initials = initials( &format!( "{} {}", forenames, surname ) );
				if let Some( title ) = &self.title {
					name_initials.insert_str( 0, &format!( "{} ", title ) );
				};
				Some( name_initials )
			},
			NameCombo::Sign => {
				let Some( forenames ) = self.designate( NameCombo::Forenames, GrammaticalCase::Nominative ) else {
					return None;
				};
				let name = match &self.predicate {
					Some( x ) => format!( "{} {}", forenames, x ),
					None => forenames,
				};
				let mut name_initials = initials( &name );
				name_initials.push_str( &format!( " {}", self.surname ) );
				if let Some( title ) = &self.title {
					name_initials.insert_str( 0, &format!( "{} ", title ) );
				};
				Some( name_initials )
			},
			_ => {
				eprintln!( "\"{:?}\" not yet implemented.", form );
				todo!();
			},
		}
	}
}




//=============================================================================
// Testing


#[cfg( test )]
mod tests {
	use super::*;

	#[test]
	fn gender_title() {
		assert_eq!( Gender::Male.polite().unwrap(), "Herr".to_string() );
		assert_eq!( Gender::Female.polite().unwrap(), "Frau".to_string() );
		assert!( Gender::Neutral.polite().is_none() );
		assert!( Gender::Other.polite().is_none() );
	}

	#[test]
	fn gender_symbol() {
		assert_eq!( Gender::Male.to_string(), "♂".to_string() );
		assert_eq!( Gender::Female.to_string(), "♀".to_string() );
		assert_eq!( Gender::Neutral.to_string(), "⚪".to_string() );
		assert_eq!( Gender::Other.to_string(), "⚧".to_string() );
	}

	#[test]
	fn test_initials() {
		assert_eq!( initials( "Test Test"), "T. T.".to_string() );
		assert_eq!( initials( "Thomas von Würzinger"), "T. v. W.".to_string() );
	}

	#[test]
	fn name_strings_male() {
		// Thomas Jakob von Würzinger
		let name = Names {
			forenames: [ "Thomas", "Jakob" ].iter().map( |x| x.to_string() ).collect(),
			predicate: Some( "von".to_string() ),
			surname: "Würzinger".to_string(),
			birthname: None,
			title: None,
			rank: Some( "Hauptkommissar".to_string() ),
			nickname: Some( "Würzi".to_string() ),
			honorname: Some( "Dunkle".to_string() ),
			supername: None,
			gender: Gender::Male,
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
	}

	#[test]
	fn name_strings_female() {
		// Penelope Karin von Würzinger geb. Stauff
		let name = Names {
			forenames: [ "Penelope", "Karin" ].iter().map( |x| x.to_string() ).collect(),
			predicate: Some( "von".to_string() ),
			surname: "Würzinger".to_string(),
			birthname: Some( "Stauff".to_string() ),
			title: Some( "Dr.".to_string() ),
			rank: Some( "Majorin".to_string() ),
			nickname: None,
			honorname: Some( "Große".to_string() ),
			supername: None,
			gender: Gender::Female,
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
			surname: "Julius".to_string(),
			birthname: None,
			title: None,
			rank: None,
			nickname: Some( "Caesar".to_string() ),
			honorname: None,
			supername: None,
			gender: Gender::Male,
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
			surname: "Iunia".to_string(),
			birthname: None,
			title: None,
			rank: None,
			nickname: Some( "Prima".to_string() ),
			honorname: None,
			supername: None,
			gender: Gender::Female,
		};

		assert_eq!(
			name.designate( NameCombo::DuaNomina, GrammaticalCase::Nominative ).unwrap(),
			"Iunia Prima".to_string()
		);
	}
}
