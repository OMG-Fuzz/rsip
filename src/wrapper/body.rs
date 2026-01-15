use std::sync::Arc;

use aglaea::{
    exp, new_node, ntc, t_bytes_val, DerivationTree, Grammar, PangLabel, ToGrammar, ToTree,
    ToTreeAs,
};

// BodyWrapper for SIP message body (Vec<u8>)
#[derive(Debug, PartialEq, Clone, PangLabel)]
pub struct BodyWrapper(pub Vec<u8>);

impl ToGrammar for BodyWrapper {
    fn grammar_with_context(visited: &mut std::collections::HashSet<String>) -> Grammar {
        let mut rules = Grammar::new();

        let my_label = Self::label().to_string();

        if !visited.insert(my_label.clone()) {
            return rules;
        }

        // BodyWrapper -> any bytes (can be empty)
        rules.insert(
            Self::label(),
            vec![exp(vec![t_bytes_val(&[])])], // Placeholder for any body content
        );

        rules
    }
}

impl ToTree for BodyWrapper {
    fn to_tree(&self) -> Arc<DerivationTree> {
        new_node(ntc(
            &Self::label(),
            Some(vec![new_node(t_bytes_val(&self.0))]),
        ))
    }
}

impl From<Vec<u8>> for BodyWrapper {
    fn from(body: Vec<u8>) -> Self {
        BodyWrapper(body)
    }
}

impl ToTreeAs<BodyWrapper> for Vec<u8> {
    fn to_tree_as(&self) -> Arc<DerivationTree> {
        BodyWrapper(self.clone()).to_tree()
    }
}

impl ToTreeAs<BodyWrapper> for &Vec<u8> {
    fn to_tree_as(&self) -> Arc<DerivationTree> {
        BodyWrapper((*self).clone()).to_tree()
    }
}
