use super::modifier::Modifier;
use cons::Cons;
use into_tokens::IntoTokens;
use swift::Swift;
use tokens::Tokens;
use Java;

/// Model for Swift Arguments to functions.
#[derive(Debug, Clone)]
pub struct Argument<'el> {
    /// Type of argument.
    ty: Swift<'el>,
    /// Name of argument.
    name: Cons<'el>,

    initializer: Tokens<'el, Swift<'el>>,
}

impl<'el> Argument<'el> {
    /// Build a new empty argument.
    pub fn new<T, N>(ty: T, name: N) -> Argument<'el>
    where
        T: Into<Swift<'el>>,
        N: Into<Cons<'el>>,
    {
        Argument {
            ty: ty.into(),
            name: name.into(),
            initializer: Tokens::new(),
        }
    }

    /// Set the initializer for argument.
    pub fn initializer<I>(&mut self, initializer: I) where I : IntoTokens<'el, Swift<'el>> {
        self.initializer.append(initializer.into_tokens())
    }

    /// Get the variable of the argument.
    pub fn var(&self) -> Cons<'el> {
        self.name.clone()
    }

    /// The type of the argument.
    pub fn ty(&self) -> Swift<'el> {
        self.ty.clone()
    }
}

into_tokens_impl_from!(Argument<'el>, Swift<'el>);

impl<'el> IntoTokens<'el, Swift<'el>> for Argument<'el> {
    fn into_tokens(self) -> Tokens<'el, Swift<'el>> {
        let mut s = Tokens::new();
        s.append(self.name);
        s.append(":");
        s.append(self.ty);
        if !self.initializer.is_empty() {
            s.append("=");
            s.extend(self.initializer);
        }

        s.join_spacing()
    }
}

#[cfg(test)]
mod tests {
    use cons::Cons;
    use swift::argument::Argument;
    use swift::Swift::Type;
    use swift::{local, Name, Swift};
    use tokens::Tokens;

    #[test]
    fn test_argument() {
        let mut c = Argument::new(
            local("Int"),
            "arg",
        );
        let mut init = Tokens::new();
        init.append("100");
        c.initializer(init);

        let t: Tokens<Swift> = c.into();

        let s = t.to_string();
        let out = s.as_ref().map(|s| s.as_str());
        assert_eq!(Ok("arg : Int = 100"), out);
    }
}
