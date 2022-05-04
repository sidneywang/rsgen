//! Data structure for fields

use con_::Con;
use element::Element::Spacing;
use swift::comment::BlockComment;
use swift::modifier::Modifier;
use swift::Swift;
use {Cons, Tokens};
use {Element, IntoTokens};

/// Model for Swift Fields.
#[derive(Debug, Clone)]
pub struct Field<'el> {
    /// Modifiers of field.
    pub modifiers: Vec<Modifier>,
    /// Comments associated with this field.
    pub comments: Vec<Cons<'el>>,
    /// Type of field.
    ty: Swift<'el>,
    /// Name of field.
    name: Cons<'el>,
    /// Initializer of field.
    initializer: Option<Tokens<'el, Swift<'el>>>,
    /// If it is mutable.
    mutable: bool,
    /// Getter for properties
    getter: Option<Tokens<'el, Swift<'el>>>,
    /// Setter for properties
    setter: Option<Tokens<'el, Swift<'el>>>,
}

impl<'el> Field<'el> {
    /// Create a new field.
    pub fn new<T, N>(ty: T, name: N) -> Field<'el>
    where
        T: Into<Swift<'el>>,
        N: Into<Cons<'el>>,
    {
        use self::Modifier::*;

        Field {
            modifiers: vec![Private],
            comments: vec![],
            ty: ty.into(),
            name: name.into(),
            initializer: None,
            mutable: false,
            getter: None,
            setter: None,
        }
    }

    /// Set initializer for field.
    pub fn initializer<I>(&mut self, initializer: I)
    where
        I: IntoTokens<'el, Swift<'el>>,
    {
        self.initializer = Some(initializer.into_tokens());
    }

    /// Set mutable for the field.
    pub fn mutable(&mut self, mutable: bool) {
        self.mutable = mutable;
    }

    /// The variable of the field.
    pub fn var(&self) -> Cons<'el> {
        self.name.clone()
    }

    /// The type of the field.
    pub fn ty(&self) -> Swift<'el> {
        self.ty.clone()
    }
}

into_tokens_impl_from!(Field<'el>, Swift<'el>);

impl<'el> IntoTokens<'el, Swift<'el>> for Field<'el> {
    fn into_tokens(self) -> Tokens<'el, Swift<'el>> {
        let mut tokens = Tokens::new();

        tokens.push_unless_empty(BlockComment(self.comments));

        tokens.append({
            let mut sig = Tokens::new();
            sig.extend(self.modifiers.into_tokens());
            if self.mutable {
                sig.append("var")
            } else {
                sig.append("let")
            }
            sig.append(self.name);
            sig.append(":");
            sig.append(self.ty);

            if let Some(initializer) = self.initializer {
                sig.append("=");
                sig.append(initializer);
            }

            sig.join_spacing()
        });

        if self.getter.is_some() || self.setter.is_some() {
            tokens.append(Spacing);
            tokens.append("{");
            tokens.nested({
                let mut body = Tokens::new();
                if let Some(getter) = self.getter {
                    body.push("get");
                    if !getter.is_empty() {
                        body.append(Spacing);
                        body.append("{");
                        body.push(getter);
                        body.push("}");
                    }
                }

                if let Some(setter) = self.setter {
                    body.push("set");
                    if !setter.is_empty() {
                        body.append(Spacing);
                        body.append("{");
                        body.push(setter);
                        body.push("}");
                    }
                }
                body
            });
            tokens.push("}");
        }

        tokens
    }
}

impl<'el> From<Field<'el>> for Element<'el, Swift<'el>> {
    fn from(f: Field<'el>) -> Self {
        Element::Append(Con::Owned(f.into_tokens()))
    }
}

#[cfg(test)]
mod tests {
    use swift::field::Field;
    use swift::local;
    use tokens::Tokens;

    fn field() -> Field<'static> {
        Field::new(local("Int"), "foo")
    }

    #[test]
    fn test_with_comments() {
        let mut c = field();
        c.comments.push("Hello World".into());
        let t: Tokens<_> = c.into();
        assert_eq!(
            Ok(String::from(
                "/**\n * Hello World\n */\nprivate let foo : Int",
            )),
            t.to_string()
        );
    }

    #[test]
    fn test_no_comments() {
        let t = Tokens::from(field());
        assert_eq!(Ok(String::from("private let foo : Int")), t.to_string());
    }

    #[test]
    fn test_field() {
        let mut field = Field::new(local("Int"), "foo");
        field.mutable = true;
        field.initializer("300");
        field.getter = Some(Tokens::new());
        field.setter = Some(Tokens::new());
        let t: Tokens<_> = field.into();
        let result = t.to_string();
        assert_eq!(
            Ok(String::from(
                "private var foo : Int = 300 {\n  get\n  set\n}"
            )),
            result
        );
    }
}
