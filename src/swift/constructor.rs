//! Data structure for constructors

use super::argument::Argument;
use super::modifier::Modifier;
use con_::Con::Owned;
use cons::Cons;
use element::Element;
use into_tokens::IntoTokens;
use swift::Swift;
use tokens::Tokens;

/// Model for Java Constructors.
#[derive(Debug, Clone)]
pub struct Constructor<'el> {
    /// Constructor modifiers.
    pub modifiers: Vec<Modifier>,
    /// Arguments for the constructor.
    pub arguments: Vec<Argument<'el>>,
    /// Body of the constructor.
    pub body: Tokens<'el, Swift<'el>>,
    /// Exception thrown by the constructor.
    pub throws: bool,
}

impl<'el> Constructor<'el> {
    /// Build a new empty constructor.
    pub fn new() -> Constructor<'el> {
        Constructor {
            modifiers: vec![Modifier::Public],
            arguments: Vec::new(),
            throws: false,
            body: Tokens::new(),
        }
    }
}

into_tokens_impl_from!(Constructor<'el>, Swift<'el>);

impl<'el> IntoTokens<'el, Swift<'el>> for Constructor<'el> {
    fn into_tokens(self) -> Tokens<'el, Swift<'el>> {
        use self::Element::*;

        let mut c = self;

        let args: Vec<Tokens<Swift>> = c.arguments.into_iter().map(|a| a.into_tokens()).collect();
        let args: Tokens<Swift> = args.into_tokens();

        let mut sig: Tokens<Swift> = Tokens::new();

        c.modifiers.sort();
        sig.extend(c.modifiers.into_iter().map(Into::into));

        if !args.is_empty() {
            let sep = toks![",", PushSpacing];
            let args = args.join(sep);

            sig.append(toks!["init", "(", Nested(Owned(args)), ")",]);
        } else {
            sig.append(toks!["init", "()"]);
        }

        if c.throws {
            sig.append("throws");
        }

        let mut s = Tokens::new();

        s.push(toks![sig.join_spacing(), " {"]);
        s.nested(c.body);
        s.push("}");

        s
    }
}

#[cfg(test)]
mod tests {
    use super::Constructor;
    use cons::Cons;
    use swift::Swift;
    use tokens::Tokens;

    #[test]
    fn test_construct() {
        let c = Constructor::new();
        let t: Tokens<Swift> = c.into();

        let s = t.to_string();
        let out = s.as_ref().map(|s| s.as_str());
        assert_eq!(Ok("public init() {\n}"), out);
    }

    #[test]
    fn test_throws() {
        let mut c = Constructor::new();
        c.throws = true;
        let t: Tokens<Swift> = c.into();

        let s = t.to_string();
        let out = s.as_ref().map(|s| s.as_str());
        assert_eq!(Ok("public init() throws {\n}"), out);
    }
}
