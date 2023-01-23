/*!
Extension methods
*/

use crate::SymbolMetadata;

trait MyVec {
    /// Searches the list by symbol ("BVME:DEM") and returns an option.
    fn find_by_full_symbol(&self, full_symbol: &str) -> Option<&SymbolMetadata>;
}

impl MyVec for Vec<SymbolMetadata> {
    fn find_by_full_symbol(&self, full_symbol: &str) -> Option<&SymbolMetadata> {
        self.iter().find_map(|sm| {
            if sm.symbol_w_namespace() == full_symbol {
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
}
