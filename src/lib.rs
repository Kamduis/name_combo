//! This crate provides the means to save and represent person's names.
//!
//! In its current state this crate concentrates on german names but can be used to represent a variety of names of different languages.




//=============================================================================
// Helper functions


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
pub enum GrammaticalCase {
	Nominative,
	Genetive,
	Dative,
	Accusative,
}


/// The possible combination of names.
pub enum NameCombo {
	/// The first forename. Bsp.: "Thomas"
	Firstname,

	/// All forenames. Bsp.: "Thomas Jakob"
	Forenames,

	/// The full name. Bsp.: "Penelope Karin von Würzinger geb. Stauff"
	Fullname,

	/// Only the full surname. This includes all name predicates. Bsp.: "von Würzinger"
	Surname,

	/// This represents the standard (german) name combination of first name and surname. Bsp.: "Penelope von Würzinger"
	Name,
}




//=============================================================================
// Structs


/// The different names of a person that can be combined in various ways.
pub struct Names {
	forenames: Vec<String>,
	predicate: Option<String>,
	surname: String,
	birthname: Option<String>,
}

impl Names {
	/// Returns all fornames as a string. Bsp. "Thomas Jakob".
	fn forenames( &self ) -> String {
		self.forenames.join( " " )
	}

	/// Returns the full surname including all predicates. Bsp. "von Würzinger".
	fn surname_full( &self ) -> String {
		match &self.predicate {
			Some( x ) => format!( "{} {}", x, self.surname ),
			None => self.surname.clone(),
		}
	}

	/// Returns a calling of a name.
	pub fn name( &self, form: NameCombo, case: GrammaticalCase ) -> String {
		let forenames = self.forenames();
		let surname_full = self.surname_full();

		match form {
			NameCombo::Name => {
				let parts = [ self.forenames[0].as_str(), surname_full.as_str() ];
				let text = parts.iter()
					.map( |x| *x )
					.collect::<Vec<&str>>()
					.join( " " );
				add_case_letter( &text, case )
			},
			NameCombo::Surname => add_case_letter( &self.surname_full(), case ),
			NameCombo::Firstname => add_case_letter( &self.forenames[0], case ),
			NameCombo::Forenames => add_case_letter( &self.forenames(), case ),
			NameCombo::Fullname => {
				let parts = [
					forenames.as_str(),
					surname_full.as_str(),
				];
				let text = parts.iter()
					.map( |x| *x )
					.collect::<Vec<&str>>()
					.join( " " );
				let name = add_case_letter( &text, case );
				match &self.birthname {
					Some( x ) => format!( "{} geb. {}", name, x ),
					None => name,
				}
			},
			_ => todo!(),
		}
	}
}




//=============================================================================
// Testing


#[cfg( test )]
mod tests {
	use super::*;

	#[test]
	fn name_strings_male() {
		// Thomas Jakob von Würzinger
		let name = Names {
			forenames: [ "Thomas", "Jakob" ].iter().map( |x| x.to_string() ).collect(),
			predicate: Some( "von".to_string() ),
			surname: "Würzinger".to_string(),
			birthname: None,
			nickname: None,
			supername: None,
		};

		assert_eq!(
			name.name( NameCombo::Name, GrammaticalCase::Nominative ),
			"Thomas von Würzinger".to_string()
		);
		assert_eq!(
			name.name( NameCombo::Name, GrammaticalCase::Genetive ),
			"Thomas von Würzingers".to_string()
		);
		assert_eq!(
			name.name( NameCombo::Name, GrammaticalCase::Accusative ),
			"Thomas von Würzinger".to_string()
		);

		assert_eq!(
			name.name( NameCombo::Surname, GrammaticalCase::Nominative ),
			"von Würzinger".to_string()
		);

		assert_eq!(
			name.name( NameCombo::Firstname, GrammaticalCase::Nominative ),
			"Thomas".to_string()
		);
		assert_eq!(
			name.name( NameCombo::Firstname, GrammaticalCase::Genetive ),
			"Thomas'".to_string()
		);

		assert_eq!(
			name.name( NameCombo::Forenames, GrammaticalCase::Nominative ),
			"Thomas Jakob".to_string()
		);

		assert_eq!(
			name.name( NameCombo::Fullname, GrammaticalCase::Nominative ),
			"Thomas Jakob von Würzinger".to_string()
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
			nickname: None,
			supername: None,
		};

		assert_eq!(
			name.name( NameCombo::Name, GrammaticalCase::Nominative ),
			"Penelope von Würzinger".to_string()
		);
		assert_eq!(
			name.name( NameCombo::Name, GrammaticalCase::Genetive ),
			"Penelope von Würzingers".to_string()
		);
		assert_eq!(
			name.name( NameCombo::Name, GrammaticalCase::Accusative ),
			"Penelope von Würzinger".to_string()
		);

		assert_eq!(
			name.name( NameCombo::Surname, GrammaticalCase::Nominative ),
			"von Würzinger".to_string()
		);

		assert_eq!(
			name.name( NameCombo::Firstname, GrammaticalCase::Nominative ),
			"Penelope".to_string()
		);

		assert_eq!(
			name.name( NameCombo::Fullname, GrammaticalCase::Nominative ),
			"Penelope Karin von Würzinger geb. Stauff".to_string()
		);
	}
}
