#[doc(hidden)]
pub use tokenizer::Tokenizer;

use crate::Error;
use std::convert::TryFrom;

macro_rules! create_methods {
    ($($name:ident),*) => {

        /// The SIP [Request](super::super::Request) method.
        #[derive(Debug, PartialEq, Eq, Clone, Copy, aglaea::PangLabel)]
        pub enum Method {
            $(
                $name,
            )*
        }

        impl Method {
            pub fn all() -> Vec<Method> {
                vec![
                    $(
                        Self::$name,
                    )*
                ]
            }
        }

        impl std::fmt::Display for Method {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Self::$name => write!(f, "{}", stringify!($name).to_uppercase()),
                    )*
                }
            }
        }

        impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a str, char>> for Method {
            type Error = Error;

            fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a str, char>) -> Result<Self, Self::Error> {
                match tokenizer.value {
                    $(
                        part if part.trim().eq_ignore_ascii_case(stringify!($name)) => Ok(Method::$name),
                    )*
                    part => Err(Error::ParseError(format!("invalid method: {}", part))),
                }
            }
        }
    }
}

create_methods!(
    Ack, Bye, Cancel, Info, Invite, Message, Notify, Options, PRack, Publish, Refer, Register,
    Subscribe, Update
);

//TODO: not ideal performance here
impl std::str::FromStr for Method {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(Tokenizer::from(s))
    }
}

impl<'a> std::convert::TryFrom<tokenizer::Tokenizer<'a, &'a [u8], u8>> for Method {
    type Error = Error;

    fn try_from(tokenizer: tokenizer::Tokenizer<'a, &'a [u8], u8>) -> Result<Self, Self::Error> {
        use std::str::from_utf8;

        let value = from_utf8(tokenizer.value)?;

        Method::try_from(Tokenizer::from(value))
    }
}

#[doc(hidden)]
pub mod tokenizer {
    use crate::{AbstractInput, AbstractInputItem, GResult, GenericNomError, TokenizerError};
    use std::marker::PhantomData;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        pub value: T,
        phantom1: PhantomData<&'a T>,
        phantom2: PhantomData<I>,
    }

    impl<'a, T, I> From<T> for Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        fn from(value: T) -> Self {
            Self {
                value,
                phantom1: Default::default(),
                phantom2: Default::default(),
            }
        }
    }

    impl<'a, T, I> Tokenizer<'a, T, I>
    where
        T: AbstractInput<'a, I>,
        I: AbstractInputItem<I>,
    {
        //works for request line
        pub fn tokenize(part: T) -> GResult<T, Self> {
            use nom::bytes::complete::{tag, take_while1};

            let (rem, method) =
                take_while1(I::is_token)(part).map_err(|_: GenericNomError<'a, T>| {
                    TokenizerError::from(("method", part)).into()
                })?;
            //TODO: helpful to return early in case we parse a response but maybe it should not
            //be checked here though
            match tag::<_, _, nom::error::VerboseError<T>>("SIP/")(method) {
                Err(_) => Ok((rem, Self::from(method))),
                Ok(_) => Err(TokenizerError::from(("method", part)).into()),
            }
        }
    }
}

// Aglaea implementations for Method
impl aglaea::ToGrammar for Method {
    fn grammar_with_context(_visited: &mut std::collections::HashSet<String>) -> aglaea::Grammar {
        use aglaea::{exp, t_bytes_val, PangLabel};
        let mut rules = aglaea::Grammar::new();

        // Create alternative rules for each method
        rules.insert(
            Self::label(),
            vec![
                exp(vec![t_bytes_val(b"ACK")]),
                exp(vec![t_bytes_val(b"BYE")]),
                exp(vec![t_bytes_val(b"CANCEL")]),
                exp(vec![t_bytes_val(b"INFO")]),
                exp(vec![t_bytes_val(b"INVITE")]),
                exp(vec![t_bytes_val(b"MESSAGE")]),
                exp(vec![t_bytes_val(b"NOTIFY")]),
                exp(vec![t_bytes_val(b"OPTIONS")]),
                exp(vec![t_bytes_val(b"PRACK")]),
                exp(vec![t_bytes_val(b"PUBLISH")]),
                exp(vec![t_bytes_val(b"REFER")]),
                exp(vec![t_bytes_val(b"REGISTER")]),
                exp(vec![t_bytes_val(b"SUBSCRIBE")]),
                exp(vec![t_bytes_val(b"UPDATE")]),
            ],
        );

        rules
    }
}

impl aglaea::ToTree for Method {
    fn to_tree(&self) -> std::sync::Arc<aglaea::DerivationTree> {
        use aglaea::{new_node, ntc, t_bytes_val, PangLabel};

        let method_bytes = self.to_string().into_bytes();
        new_node(ntc(
            &Self::label(),
            Some(vec![new_node(t_bytes_val(&method_bytes))]),
        ))
    }
}

#[cfg(feature = "test-utils")]
impl testing_utils::Randomize for Method {
    fn random() -> Self {
        testing_utils::sample_vec(&Self::all())
    }
}
