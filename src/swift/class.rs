//! Data structure for classes.

use swift::constructor::Constructor;
use swift::field::Field;
use swift::method::Method;
use swift::modifier::Modifier;
use ::{Cons, IntoTokens};
use ::{Element, Tokens};
use swift::Swift;

/// Model for Swift Classs.
#[derive(Debug, Clone)]
pub struct Class<'el> {
    /// Class modifiers.
    pub modifiers: Vec<Modifier>,
    /// Declared methods.
    pub fields: Vec<Field<'el>>,
    /// Declared methods.
    pub constructors: Vec<Constructor<'el>>,
    /// Declared methods.
    pub methods: Vec<Method<'el>>,
    /// What this class implements.
    pub implements: Vec<Swift<'el>>,
    /// Generic parameters.
    pub parameters: Tokens<'el, Swift<'el>>,
    /// Annotations for the constructor.
    attributes: Tokens<'el, Swift<'el>>,
    /// Name of class.
    name: Cons<'el>,
}

impl<'el> Class<'el> {
    /// Build a new empty interface.
    pub fn new<N>(name: N) -> Class<'el>
    where
        N: Into<Cons<'el>>,
    {
        Class {
            modifiers: vec![Modifier::Public],
            fields: vec![],
            methods: vec![],
            constructors: vec![],
            implements: vec![],
            parameters: Tokens::new(),
            attributes: Tokens::new(),
            name: name.into(),
        }
    }

    /// Push an annotation.
    pub fn attributes<A>(&mut self, attribute: A)
    where
        A: IntoTokens<'el, Swift<'el>>,
    {
        self.attributes.push(attribute.into_tokens());
    }

    /// Name of class.
    pub fn name(&self) -> Cons<'el> {
        self.name.clone()
    }
}

into_tokens_impl_from!(Class<'el>, Swift<'el>);

impl<'el> IntoTokens<'el, Swift<'el>> for Class<'el> {
    fn into_tokens(self) -> Tokens<'el, Swift<'el>> {
        let mut sig = Tokens::new();

        sig.extend(self.modifiers.into_tokens());
        sig.append("class");

        sig.append({
            let mut t = Tokens::new();

            t.append(self.name.clone());

            if !self.parameters.is_empty() {
                t.append("<");
                t.append(self.parameters.join(", "));
                t.append(">");
            }

            t
        });

        if !self.implements.is_empty() {
            let implements: Tokens<_> = self
                .implements
                .into_iter()
                .map::<Element<_>, _>(Into::into)
                .collect();

            sig.append(":");
            sig.append(implements.join(", "));
        }

        let mut s = Tokens::new();

        if !self.attributes.is_empty() {
            s.push(self.attributes);
        }

        s.push(toks![sig.join_spacing(), " {"]);

        s.nested({
            let mut body = Tokens::new();

            if !self.fields.is_empty() {
                for field in self.fields {
                    body.push(field);
                }
            }

            if !self.constructors.is_empty() {
                for constructor in self.constructors {
                    body.push(constructor);
                }
            }

            if !self.methods.is_empty() {
                for method in self.methods {
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
    use swift::class::Class;
    use swift::{local, Swift};
    use Tokens;

    #[test]
    fn test_vec() {
        let mut c = Class::new("Foo");
        c.parameters.append("T");
        c.implements = vec![local("Super").into()];

        let t: Tokens<Swift> = c.into();

        let s = t.to_string();
        let out = s.as_ref().map(|s| s.as_str());
        assert_eq!(Ok("public class Foo<T> : Super {\n}"), out);
    }
}
