/// A Swift modifier.

use std::collections::BTreeSet;
use {Custom, Element, IntoTokens, Tokens};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Modifier {
    Open,
    Public,
    Internal,
    FilePrivate,
    Private,
    Static,
    Final,
    Class,
    Mutating,
    Throws,
    Convenience,
    Override,
    Required
}

impl Modifier {
    pub fn name(&self) -> &'static str {
        use self::Modifier::*;
        match *self {
            Open => "open",
            Public => "public",
            Internal => "internal",
            FilePrivate => "fileprivate",
            Private => "private",
            Static => "static",
            Final => "final",
            Class => "class",
            Mutating => "mutating",
            Throws => "throws",
            Convenience => "convenience",
            Override => "override",
            Required => "required"
        }
    }
}

impl<'el, C: Custom> From<Modifier> for Element<'el, C> {
    fn from(value: Modifier) -> Self {
        value.name().into()
    }
}

impl<'el, C: Custom> IntoTokens<'el, C> for Vec<Modifier> {
    fn into_tokens(self) -> Tokens<'el, C> {
        self.into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(Element::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Modifier;
    use swift::Swift;
    use tokens::Tokens;

    #[test]
    fn test_vec() {
        use self::Modifier::*;
        let el: Tokens<Swift> = toks![Public, Static, Final].join_spacing();
        let s = el.to_string();
        let out = s.as_ref().map(|s| s.as_str());
        assert_eq!(Ok("public static final"), out);
    }
}