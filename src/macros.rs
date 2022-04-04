//! Macros in RsGen

/// Helper macro to reduce boilerplate needed with nested token expressions.
///
/// ## Examples
///
/// ```rust,ignore
/// let n1: rsgen::Tokens<()> = toks!("var v = ", "bar".quoted(), ";");
/// ```
#[macro_export]
macro_rules! toks {
    ($($x:expr),*) => {
        {
            let mut _t = $crate::Tokens::new();
            $(_t.append($x);)*
            _t
        }
    };

    ($($x:expr,)*) => {toks!($($x),*)}
}

/// Helper macro to reduce boilerplate needed with pushed token expressions.
///
/// All arguments being pushed are cloned, which should be cheap for reference types.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use] extern crate rsgen;
/// # fn main() {
/// use rsgen::{Tokens, Java, Cons};
///
/// let mut toks = Tokens::<Java>::new();
/// // id being cloned.
/// let id = Cons::from(String::from("hello"));
///
/// push!(toks, "foo ", id);
/// push!(toks, "bar ", id);
///
/// let mut out = Vec::new();
/// out.push("foo hello");
/// out.push("bar hello");
///
/// assert_eq!(out.join("\n").as_str(), toks.to_string().unwrap().as_str());
/// # }
/// ```
///
/// Pushing as a block:
///
/// ```rust
/// # #[macro_use] extern crate rsgen;
/// # fn main() {
/// use rsgen::{Tokens, Java, Cons};
///
/// let mut toks = Tokens::<Java>::new();
/// // id being cloned.
/// let id = Cons::from(String::from("hello"));
///
/// push!(toks, |t| {
///   push!(t, "foo ", id);
///   push!(t, "bar ", id);
/// });
///
/// let mut out = Vec::new();
/// out.push("foo hello");
/// out.push("bar hello");
///
/// assert_eq!(out.join("\n").as_str(), toks.to_string().unwrap().as_str());
/// # }
/// ```
#[macro_export]
macro_rules! push {
    ($dest:expr, |$t:ident| $code:block) => {
        $dest.push({
            let mut $t = $crate::Tokens::new();
            $code
            $t
        })
    };

    ($dest:expr, $($x:expr),*) => {
        $dest.push({
            let mut _t = $crate::Tokens::new();
            $(_t.append(Clone::clone(&$x));)*
            _t
        })
    };

    ($dest:expr, $($x:expr,)*) => {push!($dest, $($x),*)};
}

///
///        let v = "v";
//         let a = "a";
//         let b = "b";
//
//         push_f!(t, |t| {
//             push_f!(t, "function bar({}, {}) {{", "a", "b");
//             nested_f!(t, |t| {
//                 push_f!(t, "var {} = {} + {};", v, a, b);
//                 push_f!(t, "return v;");
//             });
//             push_f!(t, "}");
//         });
//         push_f!(t, "var foo = bar();");
///
///

#[macro_export]
macro_rules! push_f {
    ($dest:expr, |$t:ident| $code:block) => {
        $dest.push({
            let mut $t = $crate::Tokens::new();
            $code
            $t
        })
    };

    ($dest:expr, $f:expr) => {
        $dest.push({
            let mut _t = $crate::Tokens::new();
            _t.append(Clone::clone(&$f));
            _t
        })
    };

    ($dest:expr, $f:expr, $($x:expr),*) => {
        $dest.push({
            let mut _t = $crate::Tokens::new();
            let fmt = format!($f, $(Clone::clone(&$x)),*);
            _t.append(fmt);
            _t
        })
    };

    ($dest:expr, $f:expr, $($x:expr,)*) => {push_f!($dest, $f, $($x),*)};
}
///
///        let v = "v";
//         let a = "a";
//         let b = "b";
//
//         push_f!(t, |t| {
//             push_f!(t, "function bar({}, {}) {{", "a", "b");
//             nested_f!(t, |t| {
//                 push_f!(t, "var {} = {} + {};", v, a, b);
//                 push_f!(t, "return v;");
//             });
//             push_f!(t, "}");
//         });
//         push_f!(t, "var foo = bar();");
///
///
#[macro_export]
macro_rules! nested_f {
    ($dest:expr, |$t:ident| $code:block) => {
        $dest.nested({
            let mut $t = $crate::Tokens::new();
            $code
            $t
        })
    };

    ($dest:expr, $f:expr) => {
        $dest.nested({
            let mut _t = $crate::Tokens::new();
            _t.append(Clone::clone(&$f));
            _t
        })
    };

    ($dest:expr, $f:expr, $($x:expr),*) => {
        $dest.nested({
            let mut _t = $crate::Tokens::new();
            let fmt = format!($f, $(Clone::clone(&$x)),*);
            _t.append(fmt);
            _t
        })
    };

    ($dest:expr, $f:expr, $($x:expr,)*) => {nested_f!($dest, $f, $($x),*)};
}

/// Helper macro to reduce boilerplate needed with nested token expressions.
///
/// All arguments being pushed are cloned, which should be cheap for reference types.
///
/// ## Examples
///
/// ```rust
/// # #[macro_use] extern crate rsgen;
/// # fn main() {
/// use rsgen::{Tokens, Java, Cons};
///
/// let mut toks = Tokens::<Java>::new();
/// // id being cloned.
/// let id = Cons::from(String::from("hello"));
///
/// nested!(toks, "foo ", id);
/// nested!(toks, "bar ", id);
///
/// let mut out = Vec::new();
/// out.push("  foo hello");
/// out.push("  bar hello");
/// out.push("");
///
/// assert_eq!(out.join("\n").as_str(), toks.to_string().unwrap().as_str());
/// # }
/// ```
///
/// Pushing as a block:
///
/// ```rust
/// # #[macro_use] extern crate rsgen;
/// # fn main() {
/// use rsgen::{Tokens, Java, Cons};
///
/// let mut toks = Tokens::<Java>::new();
/// // id being cloned.
/// let id = Cons::from(String::from("hello"));
///
/// nested!(toks, |t| {
///   push!(t, "foo ", id);
///   push!(t, "bar ", id);
/// });
///
/// let mut out = Vec::new();
/// out.push("  foo hello");
/// out.push("  bar hello");
/// out.push("");
///
/// assert_eq!(out.join("\n").as_str(), toks.to_string().unwrap().as_str());
/// # }
/// ```
#[macro_export]
macro_rules! nested {
    ($dest:expr, |$t:ident| $code:block) => {
        $dest.nested({
            let mut $t = $crate::Tokens::new();
            $code
            $t
        })
    };

    ($dest:expr, $($x:expr),*) => {
        $dest.nested({
            let mut _t = $crate::Tokens::new();
            $(_t.append(Clone::clone(&$x));)*
            _t
        })
    };

    ($dest:expr, $($x:expr,)*) => {nested!($dest, $($x),*)};
}

macro_rules! into_tokens_impl_from {
    ($type:ty, $custom:ty) => {
        impl<'el> From<$type> for Tokens<'el, $custom> {
            fn from(value: $type) -> Tokens<'el, $custom> {
                value.into_tokens()
            }
        }
    };
}

macro_rules! into_tokens_impl_from_generic {
    ($type:ty) => {
        impl<'el, C> From<$type> for Tokens<'el, C> {
            fn from(value: $type) -> Tokens<'el, C> {
                value.into_tokens()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use js::JavaScript;
    use quoted::Quoted;
    use tokens::Tokens;

    #[test]
    fn test_quoted() {
        let n1: Tokens<JavaScript> = toks!("var v = ", "bar".quoted(), ";");
        assert_eq!("var v = \"bar\";", n1.to_string().unwrap().as_str());
    }

    #[test]
    fn test_macros() {
        let mut t = Tokens::<JavaScript>::new();

        push!(t, |t| {
            push!(t, "function bar(a, b) {");
            nested!(t, |t| {
                push!(t, "var v = a + b;");
                push!(t, "return v;");
            });
            push!(t, "}");
        });
        push!(t, "var foo = bar();");

        let mut out = Vec::new();
        out.push("function bar(a, b) {");
        out.push("  var v = a + b;");
        out.push("  return v;");
        out.push("}");
        out.push("var foo = bar();");

        assert_eq!(out.join("\n").as_str(), t.to_string().unwrap().as_str());

        let mut t = Tokens::<JavaScript>::new();

        let v = "v";
        let a = "a";
        let b = "b";

        push_f!(t, |t| {
            push_f!(t, "function bar({}, {}) {{", "a", "b");
            nested_f!(t, |t| {
                push_f!(t, "var {} = {} + {};", v, a, b);
                push_f!(t, "return v;");
            });
            push_f!(t, "}");
        });
        push_f!(t, "var foo = bar();");

        let mut out = Vec::new();
        out.push("function bar(a, b) {");
        out.push("  var v = a + b;");
        out.push("  return v;");
        out.push("}");
        out.push("var foo = bar();");

        assert_eq!(out.join("\n").as_str(), t.to_string().unwrap().as_str());
    }
}
