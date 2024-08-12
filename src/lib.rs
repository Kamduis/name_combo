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


use std::fmt;

use unic_langid::LanguageIdentifier;

mod gender;
pub use crate::gender::Gender;

mod name;
pub use crate::name::{NameError, GrammaticalCase, NameCombo, Names};




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
