use std::sync::Arc;

use aglaea::{
    exp, new_node, ntc, t_bytes_val, DerivationTree, Grammar, PangLabel, ToGrammar, ToTree,
    ToTreeAs,
};

use crate::Headers;

// HeadersWrapper for SIP headers serialization
#[derive(Debug, PartialEq, Clone, PangLabel)]
pub struct HeadersWrapper(pub Headers);

impl ToGrammar for HeadersWrapper {
    fn grammar_with_context(visited: &mut std::collections::HashSet<String>) -> Grammar {
        let mut rules = Grammar::new();

        let my_label = Self::label().to_string();

        if !visited.insert(my_label.clone()) {
            return rules;
        }

        // HeadersWrapper -> headers content + "\r\n"
        rules.insert(
            Self::label(),
            vec![exp(vec![
                t_bytes_val(&[]), // Placeholder for headers
                t_bytes_val(b"\r\n"),
            ])],
        );

        rules
    }
}

impl ToTree for HeadersWrapper {
    fn to_tree(&self) -> Arc<DerivationTree> {
        let headers_str = self.0.to_string();
        new_node(ntc(
            &Self::label(),
            Some(vec![
                new_node(t_bytes_val(headers_str.as_bytes())),
                new_node(t_bytes_val(b"\r\n")),
            ]),
        ))
    }
}

impl From<Headers> for HeadersWrapper {
    fn from(headers: Headers) -> Self {
        HeadersWrapper(headers)
    }
}

impl ToTreeAs<HeadersWrapper> for Headers {
    fn to_tree_as(&self) -> Arc<DerivationTree> {
        HeadersWrapper(self.clone()).to_tree()
    }
}

impl ToTreeAs<HeadersWrapper> for &Headers {
    fn to_tree_as(&self) -> Arc<DerivationTree> {
        HeadersWrapper((*self).clone()).to_tree()
    }
}
