//! Data structure for interfaces.

use swift::method::Method;
use swift::modifier::Modifier;
use swift::Swift;
use ::{Cons, Tokens};
use IntoTokens;
use swift::field::Field;

/// Model for Swift Protocol.
#[derive(Debug, Clone)]
pub struct Protocol<'el> {
    /// Interface modifiers.
    pub modifiers: Vec<Modifier>,
    /// Declared methods.
    pub methods: Vec<Method<'el>>,
    /// Declared Properties
    pub fields: Vec<Field<'el>>,
    /// What this interface extends.
    pub extends: Tokens<'el, Swift<'el>>,
    /// Generic parameters.
    pub parameters: Tokens<'el, Swift<'el>>,
    /// Annotations for the constructor.
    pub attributes: Tokens<'el, Swift<'el>>,
    /// Name of interface.
    name: Cons<'el>,
}

impl<'el> Protocol<'el> {
    /// Build a new empty interface.
    pub fn new<N>(name: N) -> Protocol<'el>
    where
        N: Into<Cons<'el>>,
    {
        Protocol {
            modifiers: vec![Modifier::Public],
            methods: vec![],
            fields: vec![],
            extends: Tokens::new(),
            parameters: Tokens::new(),
            attributes: Tokens::new(),
            name: name.into(),
        }
    }

    /// Push an annotation.
    pub fn annotation<A>(&mut self, annotation: A)
    where
        A: IntoTokens<'el, Swift<'el>>,
    {
        self.attributes.push(annotation.into_tokens());
    }

    /// Name of interface.
    pub fn name(&self) -> Cons<'el> {
        self.name.clone()
    }
}

into_tokens_impl_from!(Protocol<'el>, Swift<'el>);

impl<'el> IntoTokens<'el, Swift<'el>> for Protocol<'el> {
    fn into_tokens(self) -> Tokens<'el, Swift<'el>> {
        let mut sig = Tokens::new();

        sig.extend(self.modifiers.into_tokens());

        sig.append("protocol");

        sig.append({
            let mut n = Tokens::new();
            n.append(self.name);

            if !self.parameters.is_empty() {
                n.append("<");
                n.append(self.parameters.join(", "));
                n.append(">");
            }

            n
        });

        if !self.extends.is_empty() {
            sig.append(":");
            sig.append(self.extends.join(", "));
        }

        let mut s = Tokens::new();

        if !self.attributes.is_empty() {
            s.push(self.attributes);
        }

        s.push(toks![sig.join_spacing(), " {"]);
        s.nested({
            let mut body = Tokens::new();

            if !self.methods.is_empty() {
                for method in self.methods {
                    body.push(method);
                }
            }

            body.join_line_spacing()
        });

        s.nested({
            let mut body = Tokens::new();

            if !self.fields.is_empty() {
                for method in self.fields {
                    body.push(method);
                }
            }

            body.join_line_spacing()
        });
        s.push("}");

        s
    }
}

#[cfg(test)]
mod tests {
    use swift::{local, Swift};
    use swift::protocol::Protocol;
    use ::{IntoTokens, Tokens};

    #[test]
    fn test_vec() {
        let mut i = Protocol::new("Foo");
        i.parameters.append("T");
        i.extends = local("Super").into_tokens();

        let t: Tokens<Swift> = i.into();

        let s = t.to_string();
        let out = s.as_ref().map(|s| s.as_str());
        assert_eq!(Ok("public protocol Foo<T> : Super {\n}"), out);
    }
}
