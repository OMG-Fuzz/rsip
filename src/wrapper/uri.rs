use std::sync::Arc;

use aglaea::{
    exp, new_node, ntc, t_bytes_val, DerivationTree, Grammar, PangLabel, ToGrammar, ToTree,
    ToTreeAs,
};

use crate::Uri;

// UriWrapper for SIP URI serialization
#[derive(Debug, PartialEq, Clone, PangLabel)]
pub struct UriWrapper(pub Uri);

impl ToGrammar for UriWrapper {
    fn grammar_with_context(visited: &mut std::collections::HashSet<String>) -> Grammar {
        let mut rules = Grammar::new();

        let my_label = Self::label().to_string();

        if !visited.insert(my_label.clone()) {
            return rules;
        }

        // UriWrapper -> any content representing a URI
        // For fuzzing, we allow any bytes
        rules.insert(
            Self::label(),
            vec![exp(vec![t_bytes_val(&[])])], // Placeholder for any URI content
        );

        rules
    }
}

impl ToTree for UriWrapper {
    fn to_tree(&self) -> Arc<DerivationTree> {
        let uri_str = self.0.to_string();
        new_node(ntc(
            &Self::label(),
            Some(vec![new_node(t_bytes_val(uri_str.as_bytes()))]),
        ))
    }
}

impl From<Uri> for UriWrapper {
    fn from(uri: Uri) -> Self {
        UriWrapper(uri)
    }
}

impl ToTreeAs<UriWrapper> for Uri {
    fn to_tree_as(&self) -> Arc<DerivationTree> {
        UriWrapper(self.clone()).to_tree()
    }
}

impl ToTreeAs<UriWrapper> for &Uri {
    fn to_tree_as(&self) -> Arc<DerivationTree> {
        UriWrapper((*self).clone()).to_tree()
    }
}
