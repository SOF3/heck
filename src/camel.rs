use std::fmt;

/// This trait defines a camel case conversion.
///
/// In CamelCase, word boundaries are indicated by capital letters, including
/// the first word.
///
/// ## Example:
///
/// ```rust
/// extern crate heck;
/// fn main() {
///     
///     use heck::CamelCase;
///
///     let sentence = "We are not in the least afraid of ruins.";
///     assert_eq!(sentence.to_camel_case(), "WeAreNotInTheLeastAfraidOfRuins");
/// }
/// ```
pub trait CamelCase: ToOwned {
    /// Convert this type to camel case.
    fn to_camel_case(&self) -> Self::Owned;
}

impl CamelCase for str {
    fn to_camel_case(&self) -> String {
        AsCamelCase(self).to_string()
    }
}

/// This wrapper performs a camel case conversion in [`fmt::Display`].
///
/// ## Example:
///
/// ```
/// extern crate heck;
/// fn main() {
///     use heck::AsCamelCase;
///
///     let sentence = "We are not in the least afraid of ruins.";
///     assert_eq!(format!("{}", AsCamelCase(sentence)), "WeAreNotInTheLeastAfraidOfRuins");
/// }
/// ```
pub struct AsCamelCase<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> fmt::Display for AsCamelCase<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ::transform(self.0.as_ref(), ::capitalize, |_| Ok(()), f)
    }
}

#[cfg(test)]
mod tests {
    use super::CamelCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_camel_case(), $s2)
            }
        }
    }

    t!(test1: "CamelCase" => "CamelCase");
    t!(test2: "This is Human case." => "ThisIsHumanCase");
    t!(test3: "MixedUP_CamelCase, with some Spaces" => "MixedUpCamelCaseWithSomeSpaces");
    t!(test4: "mixed_up_ snake_case, with some _spaces" => "MixedUpSnakeCaseWithSomeSpaces");
    t!(test5: "kebab-case" => "KebabCase");
    t!(test6: "SHOUTY_SNAKE_CASE" => "ShoutySnakeCase");
    t!(test7: "snake_case" => "SnakeCase");
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "ThisContainsAllKindsOfWordBoundaries");
    t!(test9: "XΣXΣ baﬄe" => "XσxςBaﬄe");
    t!(test10: "XMLHttpRequest" => "XmlHttpRequest");
}
