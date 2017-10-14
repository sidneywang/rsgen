//! Data structure for interfaces.

use tokens::Tokens;
use java::Java;
use cons::Cons;
use super::modifier::Modifier;
use super::method::Method;
use into_tokens::IntoTokens;

/// Model for Java Interfaces.
#[derive(Debug, Clone)]
pub struct Interface<'el> {
    /// Interface modifiers.
    pub modifiers: Vec<Modifier>,
    /// Declared methods.
    pub methods: Vec<Method<'el>>,
    /// Extra body (added to end of interface).
    pub body: Tokens<'el, Java<'el>>,
    /// What this interface extends.
    pub extends: Option<Tokens<'el, Java<'el>>>,
    /// Annotations for the constructor.
    annotations: Tokens<'el, Java<'el>>,
    /// Name of interface.
    name: Cons<'el>,
}

impl<'el> Interface<'el> {
    /// Build a new empty interface.
    pub fn new<N>(name: N) -> Interface<'el>
    where
        N: Into<Cons<'el>>,
    {
        Interface {
            modifiers: vec![Modifier::Public],
            methods: vec![],
            body: Tokens::new(),
            extends: None,
            annotations: Tokens::new(),
            name: name.into(),
        }
    }

    /// Push an annotation.
    pub fn annotation<A>(&mut self, annotation: A)
    where
        A: IntoTokens<'el, Java<'el>>,
    {
        self.annotations.push(annotation.into_tokens());
    }

    /// Name of interface.
    pub fn name(&self) -> Cons<'el> {
        self.name.clone()
    }
}

into_tokens_impl_from!(Interface<'el>, Java<'el>);

impl<'el> IntoTokens<'el, Java<'el>> for Interface<'el> {
    fn into_tokens(self) -> Tokens<'el, Java<'el>> {
        let mut sig = Tokens::new();

        if !self.modifiers.is_empty() {
            sig.append(self.modifiers);
            sig.append(" ");
        }

        sig.append("interface ");
        sig.append(self.name);

        if let Some(extends) = self.extends {
            sig.append("extends ");
            sig.append(extends);
        }

        let mut s = Tokens::new();

        if !self.annotations.is_empty() {
            s.push(self.annotations);
        }

        s.push(toks![sig, " {"]);
        s.nested({
            let mut body = Tokens::new();

            if !self.methods.is_empty() {
                for method in self.methods {
                    body.push(method);
                }
            }

            body.extend(self.body);
            body.join_line_spacing()
        });
        s.push("}");

        s
    }
}

#[cfg(test)]
mod tests {
    use super::Interface;
    use java::Java;
    use tokens::Tokens;

    #[test]
    fn test_vec() {
        let i = Interface::new("Foo");
        let t: Tokens<Java> = i.into();

        let s = t.to_string();
        let out = s.as_ref().map(|s| s.as_str());
        assert_eq!(Ok("public interface Foo {\n}"), out);
    }
}
