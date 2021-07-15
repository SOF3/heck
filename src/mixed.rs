use std::fmt;

/// This trait defines a mixed case conversion.
///
/// In mixedCase, word boundaries are indicated by capital letters, excepting
/// the first word.
///
/// ## Example:
///
/// ```rust
/// extern crate heck;
/// fn main() {
///     
///     use heck::MixedCase;
///
///     let sentence = "It is we who built these palaces and cities.";
///     assert_eq!(sentence.to_mixed_case(), "itIsWeWhoBuiltThesePalacesAndCities");
/// }
/// ```
pub trait MixedCase: ToOwned {
    /// Convert this type to mixed case.
    fn to_mixed_case(&self) -> Self::Owned;
}

impl MixedCase for str {
    fn to_mixed_case(&self) -> String {
        AsMixedCase(self).to_string()
    }
}

/// This wrapper performs a mixed case conversion in [`fmt::Display`].
///
/// ## Example:
///
/// ```
/// extern crate heck;
/// fn main() {
///     use heck::AsMixedCase;
///     
///     let sentence = "It is we who built these palaces and cities.";
///     assert_eq!(format!("{}", AsMixedCase(sentence)), "itIsWeWhoBuiltThesePalacesAndCities");
/// }
/// ```
pub struct AsMixedCase<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> fmt::Display for AsMixedCase<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        ::transform(self.0.as_ref(), |s, out| {
            if first {
                first = false;
                ::lowercase(s, out)
            } else { ::capitalize(s, out) }
        }, |_| Ok(()), f)
    }
}

#[cfg(test)]
mod tests {
    use super::MixedCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_mixed_case(), $s2)
            }
        }
    }

    t!(test1: "CamelCase" => "camelCase");
    t!(test2: "This is Human case." => "thisIsHumanCase");
    t!(test3: "MixedUP CamelCase, with some Spaces" => "mixedUpCamelCaseWithSomeSpaces");
    t!(test4: "mixed_up_ snake_case, with some _spaces" => "mixedUpSnakeCaseWithSomeSpaces");
    t!(test5: "kebab-case" => "kebabCase");
    t!(test6: "SHOUTY_SNAKE_CASE" => "shoutySnakeCase");
    t!(test7: "snake_case" => "snakeCase");
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "thisContainsAllKindsOfWordBoundaries");
    t!(test9: "XΣXΣ baﬄe" => "xσxςBaﬄe");
    t!(test10: "XMLHttpRequest" => "xmlHttpRequest");
    // TODO unicode tests
}
