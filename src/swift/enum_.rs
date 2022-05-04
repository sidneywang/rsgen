//! Data structure for enums.

use swift::constructor::Constructor;
use swift::field::Field;
use swift::method::Method;
use swift::modifier::Modifier;
use swift::Swift;
use {Cons, IntoTokens};
use {Element, Tokens};

/// Model for Swift Enums.
#[derive(Debug, Clone)]
pub struct Enum<'el> {
    /// Variants of the enum.
    pub variants: Tokens<'el, Swift<'el>>,
    /// Enum modifiers.
    pub modifiers: Vec<Modifier>,
    /// Declared methods.
    pub fields: Vec<Field<'el>>,
    /// Declared methods.
    pub constructors: Vec<Constructor<'el>>,
    /// Declared methods.
    pub methods: Vec<Method<'el>>,
    /// Generic parameters.
    pub parameters: Tokens<'el, Swift<'el>>,
    /// Annotations for the constructor.
    attributes: Tokens<'el, Swift<'el>>,
    /// Name of enum.
    name: Cons<'el>,
}

impl<'el> Enum<'el> {
    /// Build a new empty interface.
    pub fn new<N>(name: N) -> Enum<'el>
    where
        N: Into<Cons<'el>>,
    {
        Enum {
            variants: Tokens::new(),
            modifiers: vec![Modifier::Public],
            fields: vec![],
            methods: vec![],
            constructors: vec![],
            attributes: Tokens::new(),
            name: name.into(),
            parameters: Tokens::new(),
        }
    }

    /// Push an annotation.
    pub fn attributes<A>(&mut self, attributes: A)
    where
        A: IntoTokens<'el, Swift<'el>>,
    {
        self.attributes.push(attributes.into_tokens());
    }

    /// Name of enum.
    pub fn name(&self) -> Cons<'el> {
        self.name.clone()
    }
}

into_tokens_impl_from!(Enum<'el>, Swift<'el>);

impl<'el> IntoTokens<'el, Swift<'el>> for Enum<'el> {
    fn into_tokens(self) -> Tokens<'el, Swift<'el>> {
        use self::Element::*;

        let mut sig = Tokens::new();

        sig.extend(self.modifiers.into_tokens());
        sig.append("enum");

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

        let mut s = Tokens::new();

        if !self.attributes.is_empty() {
            s.push(self.attributes);
        }

        s.push(toks![sig.join_spacing(), " {"]);

        s.nested({
            let mut body = Tokens::new();

            // different from class start
            if !self.variants.is_empty() {
                let sep = toks![PushSpacing];
                let variants = self.variants.join(sep);
                body.append(variants);
            }
            // different from class end

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
    use swift::enum_::Enum;
    use swift::Swift;
    use Tokens;

    #[test]
    fn test_vec() {
        let mut c = Enum::new("Foo");
        c.variants.append("case FOO(int)");
        c.variants.append("case BAR");

        let t: Tokens<Swift> = c.into();

        let s = t.to_string();
        let out = s.as_ref().map(|s| s.as_str());
        assert_eq!(
            Ok("public enum Foo {\n  case FOO(int)\n  case BAR\n}",),
            out
        );
    }
}
