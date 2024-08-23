//! You can create [Locale] from `&str`.
//! ```rust
//! use metw_locales::Locale;
//!
//! assert_eq!(Locale::en, Locale::from_str("en_US").unwrap());
//! assert_eq!(Locale::tr, Locale::from_str("tr_TR").unwrap());
//! assert_eq!(Locale::tr, Locale::from_str("tr").unwrap());
//! ```

mod macros;

locales!{
    tr, az;
    en;
    de
}


use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Unexpected, Visitor};
use std::fmt;


impl Serialize for Locale {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.to_str())
    }
}

impl<'de> Deserialize<'de> for Locale {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct LocaleVisitor;

        impl<'de> Visitor<'de> for LocaleVisitor {
            type Value = Locale;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid locale identifier")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Locale, E> {
                match Locale::from_str(value) {
                    Some(locale) => Ok(locale),
                    None => Err(de::Error::invalid_value(Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_str(LocaleVisitor)
    }
}
