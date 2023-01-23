/*!
The symbols library parses the symbols data file and returns the collection of SymbolMetadata objects.
 */
mod extensions;

use std::path::PathBuf;

use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Default, Deserialize)]
pub struct SymbolMetadata {
    /// Exchange
    pub namespace: Option<String>,
    /// Symbol at the exchange
    pub symbol: String,
    /// The currency used to express the symbol's price.
    pub currency: Option<String>,
    /// The name of the price update provider.
    pub updater: Option<String>,
    /// The symbol, as used by the updater.
    pub updater_symbol: Option<String>,
    /// The symbol, as used in the Ledger journal.
    pub ledger_symbol: Option<String>,
    /// The symbol, as used at Interactive Brokers.
    pub ib_symbol: Option<String>,
    /// Remarks
    pub remarks: Option<String>
}

#[allow(unused)]
impl SymbolMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn symbol_w_namespace(&self) -> String {
        match &self.namespace {
            Some(namespace) => format!("{}:{}", namespace, self.symbol),
            None => self.symbol.to_string(),
        }
    }
}

/// Read and parse the symbols collection.
pub fn read_symbols(path: &PathBuf) -> anyhow::Result<Vec<SymbolMetadata>> {
    //, options: Option<&ParseOptions>

    // read the file
    let mut rdr = csv::Reader::from_path(path)?;

    // parse
    // .records() = raw
    let collection: Vec<SymbolMetadata> = rdr.deserialize()
        .map(|result| match result {
            Ok(symbol) => symbol,
            Err(e) => panic!("Error deserializing: {e}"),
        })
        .collect();
    
    // Header row? Parsed by default.

    Ok(collection)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::read_symbols;

    #[test_log::test]
    fn read_test_file() {
        // println!("running in {:?}", std::env::current_dir());
        let path = PathBuf::from("tests/dummy.csv");
        println!("path: {:?}", path);

        let result = read_symbols(&path);
        assert!(result.is_ok());

        let actual = result.expect("parsed file");
        log::debug!("parsed: {:?}", actual);
        assert!(!actual.is_empty());
    }

    /// Confirm that the records get parsed into the struct
    #[test_log::test]
    fn test_parse() {
        let path = PathBuf::from("tests/dummy.csv");
        let list = read_symbols(&path).expect("parsed");

        assert_eq!(2, list.len());

        let actual = &list[0];

        assert_eq!("AUD", actual.symbol);
    }

    /// Panics if a row has less fields than the previous one.
    #[test_log::test]
    #[should_panic(expected="Error deserializing: CSV error: record 3 (line: 3, byte: 165): found record with 7 fields, but the previous record has 8 fields")]
    fn test_parsing_error() {
        let path = PathBuf::from("tests/dummy2.csv");

        let list = read_symbols(&path).expect("parsed");

        assert!(!list.is_empty());
    }
}
