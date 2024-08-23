#[macro_export]
macro_rules! locales {
    (
        $($locale: ident $(, $aliasses: ident)*);*
    ) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum Locales {
            $($locale),*
        }


        impl Locales {
            pub fn from_str(locale_str: &str) -> Option<Self> {
                fn match_str(locale_str: &str) -> Option<Locales>{
                    match locale_str {
                        $(
                            stringify!($locale) 
                            $(| stringify!($aliasses))*
                            => Some(Locales::$locale),
                        )*
                        _ => None
                    }
                }

                // direct match
                if let Some(locale) = match_str(locale_str) {
                    return Some(locale)
                }

                // match while replacing - with _
                if let Some(locale) = match_str(&locale_str.replace("-", "_")[..]) {
                    return Some(locale)
                }
                
                // match while replacing - with _ and getting country
                if let Some((locale, _)) = locale_str.replace("-", "_").split_once("_") {
                    if let Some(locale) = match_str(locale) {
                        return Some(locale)
                    }
                }

                None
            }

            pub fn to_str(&self) -> &str {
                match self {
                    $(
                        Locales::$locale => stringify!($locale),
                    )*
                }
            }
        }
    };
}
