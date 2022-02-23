//! Data structure for methods.

use {Cons, IntoTokens, Tokens};
use swift::argument::Argument;
use swift::comment::BlockComment;
use swift::modifier::Modifier;
use swift::{Swift, VOID};

/// Model for Swift Methods.
#[derive(Debug, Clone)]
pub struct Method<'el> {
    /// Method modifiers.
    pub modifiers: Vec<Modifier>,
    /// Arguments for the constructor.
    pub arguments: Vec<Argument<'el>>,
    /// Body of the constructor.
    pub body: Tokens<'el, Swift<'el>>,
    /// Return type.
    pub returns: Option<Swift<'el>>,
    /// Generic parameters.
    pub parameters: Tokens<'el, Swift<'el>>,
    /// Comments associated with this method.
    pub comments: Vec<Cons<'el>>,
    /// Exception thrown by the method.
    pub throws: bool,
    /// Annotations for the constructor.
    attributes: Tokens<'el, Swift<'el>>,
    /// Name of the method.
    name: Cons<'el>,
}

impl<'el> Method<'el> {
    /// Build a new empty constructor.
    pub fn new<N>(name: N) -> Method<'el>
        where
            N: Into<Cons<'el>>,
    {
        use self::Modifier::*;

        Method {
            modifiers: vec![Public],
            arguments: vec![],
            body: Tokens::new(),
            returns: None,
            parameters: Tokens::new(),
            comments: Vec::new(),
            throws: false,
            attributes: Tokens::new(),
            name: name.into(),
        }
    }

    /// Push an annotation.
    pub fn attribute<A>(&mut self, attribute: A)
        where
            A: IntoTokens<'el, Swift<'el>>,
    {
        self.attributes.push(attribute.into_tokens());
    }
    /// Set returns of the method.
    pub fn returns(&mut self, returns : Swift<'el>) {
        self.returns = Some(returns)
    }

    /// Name of method.
    pub fn name(&self) -> Cons<'el> {
        self.name.clone()
    }
}

into_tokens_impl_from!(Method<'el>, Swift<'el>);

impl<'el> IntoTokens<'el, Swift<'el>> for Method<'el> {
    fn into_tokens(self) -> Tokens<'el, Swift<'el>> {
        let mut sig = Tokens::new();

        sig.extend(self.modifiers.into_tokens());

        sig.append({
            let mut n = Tokens::new();

            n.append("func ");
            n.append(self.name);

            if !self.parameters.is_empty() {
                n.append(toks!["<", self.parameters.join(", "), ">"]);
            }

            let args: Vec<Tokens<Swift>> = self
                .arguments
                .into_iter()
                .map(IntoTokens::into_tokens)
                .collect();

            let args: Tokens<Swift> = args.into_tokens();

            n.append(toks!["(", args.join(", "), ")"]);

            n
        });

        if let Some(returns) = self.returns {
            if returns != VOID {
                sig.append("->");
                sig.append(returns);
            }
        }

        if self.throws {
            sig.append("throws");
        }

        let mut s = Tokens::new();

        s.push_unless_empty(BlockComment(self.comments));
        s.push_unless_empty(self.attributes);

        let sig = sig.join_spacing();

        if self.body.is_empty() {
            s.push(toks![sig, ";"]);
        } else {
            s.push(toks![sig, " {"]);
            s.nested(self.body);
            s.push("}");
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use swift::local;
    use super::Method;
    use tokens::Tokens;

    fn build_method() -> Method<'static> {
        let mut c = Method::new("foo");
        c.parameters.append("T");
        c
    }

    fn build_return_method() -> Method<'static> {
        let mut c = Method::new("foo");
        c.parameters.append("T");
        c.returns(local("Int"));
        c
    }

    #[test]
    fn test_with_comments() {
        let mut c = build_method();
        c.comments.push("Hello World".into());
        let t = Tokens::from(c);
        assert_eq!(
            Ok(String::from(
                "/**\n * Hello World\n */\npublic func foo<T>();",
            )),
            t.to_string()
        );
    }

    #[test]
    fn test_no_comments() {
        let t = Tokens::from(build_method());
        assert_eq!(Ok(String::from("public func foo<T>();")), t.to_string());
    }

    #[test]
    fn test_throws() {
        let mut m = build_method();
        m.throws = true;

        let t = Tokens::from(m);
        assert_eq!(
            Ok(String::from("public func foo<T>() throws;")),
            t.to_string()
        );
    }

    #[test]
    fn test_returns() {
        let t = Tokens::from(build_return_method());
        assert_eq!(Ok(String::from("public func foo<T>() -> Int;")), t.to_string());
    }
}
