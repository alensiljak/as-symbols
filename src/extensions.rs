/*!
Extension methods
*/

use crate::SymbolMetadata;

trait MyVec {
    /// Searches the list by symbol ("BVME:DEM") and returns an option.
    fn find_by_full_symbol(&self, search_term: &str) -> Option<&SymbolMetadata>;
    fn find_by_ledger_symbol(&self, search_term: &str) -> Option<&SymbolMetadata>;
}

impl MyVec for Vec<SymbolMetadata> {
    fn find_by_full_symbol(&self, search_term: &str) -> Option<&SymbolMetadata> {
        self.iter().find_map(|sm| {
            if sm.symbol_w_namespace() == search_term {
                Some(sm)
            } else {
                None
            }
        })
    }

    fn find_by_ledger_symbol(&self, search_term: &str) -> Option<&SymbolMetadata> {
        self.iter().find_map(|sm| {
            if sm.ledger_symbol.is_some() && sm.ledger_symbol.as_ref().unwrap() == search_term {
                Some(sm)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::SymbolMetadata;

    use super::MyVec;

    #[test_log::test]
    fn test_extension_method() {
        let mut a = SymbolMetadata::new();
        a.namespace = Some("XETRA".to_string());
        a.symbol = "EL49".to_string();

        let mut b = SymbolMetadata::new();
        b.symbol = "BND".to_string();

        let list = vec![a, b];

        let actual = list.find_by_full_symbol("XETRA:EL49").unwrap();
        log::debug!("result: {:?}", actual);

        assert_eq!("EL49", actual.symbol.as_str());
        assert!(false);
    }

    #[test_log::test]
    fn test_search_by_ledger_symbol() {
        let mut a = SymbolMetadata::new();
        a.namespace = Some("AMS".to_string());
        a.symbol = "VETY".to_string();
        a.ledger_symbol = Some("VETY_AS".to_string());

        let list = vec![a];

        let actual = list.find_by_ledger_symbol("VETY_AS").unwrap();

        assert_eq!("VETY", actual.symbol);
    }
}
