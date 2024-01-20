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
}

impl Names {
	fn surname_full( &self ) -> String {
		match &self.predicate {
			Some( x ) => format!( "{} {}", x, self.surname ),
			None => self.surname.clone(),
		}
	}

	/// Returns a calling of a name.
	pub fn name( &self, form: NameCombo, case: GrammaticalCase ) -> String {
		match form {
			NameCombo::Name => {
				let surname_full = self.surname_full();
				let parts = [ self.forenames[0].as_str(), surname_full.as_str() ];
				let text = parts.iter()
					.map( |x| *x )
					.collect::<Vec<&str>>()
					.join( " " );
				add_case_letter( &text, case )
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
	fn name_strings() {
		// Thomas Jakob von Würzinger
		let name_male = Names {
			forenames: [ "Thomas", "Jakob" ].iter().map( |x| x.to_string() ).collect(),
			predicate: Some( "von".to_string() ),
			surname: "Würzinger".to_string(),
		};

		// Penelope Karin von Würzinger geb. Stauff
		let name_female = Names {
			forenames: [ "Penelope", "Karin" ].iter().map( |x| x.to_string() ).collect(),
			predicate: Some( "von".to_string() ),
			surname: "Würzinger".to_string(),
		};

		assert_eq!(
			name_male.name( NameCombo::Name, GrammaticalCase::Nominative ),
			"Thomas von Würzinger".to_string()
		);
		assert_eq!(
			name_male.name( NameCombo::Name, GrammaticalCase::Genetive ),
			"Thomas von Würzingers".to_string()
		);
		assert_eq!(
			name_male.name( NameCombo::Name, GrammaticalCase::Accusative ),
			"Thomas von Würzinger".to_string()
		);

		assert_eq!(
			name_female.name( NameCombo::Name, GrammaticalCase::Nominative ),
			"Penelope von Würzinger".to_string()
		);
		assert_eq!(
			name_female.name( NameCombo::Name, GrammaticalCase::Genetive ),
			"Penelope von Würzingers".to_string()
		);
		assert_eq!(
			name_female.name( NameCombo::Name, GrammaticalCase::Accusative ),
			"Penelope von Würzinger".to_string()
		);
	}
}
