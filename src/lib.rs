/*!
The symbols library parses the symbols data file and returns the collection of SymbolMetadata objects.
 */

use std::path::PathBuf;

use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Default, Deserialize)]
pub struct SymbolMetadata {
    /// Exchange
    namespace: Option<String>,
    /// Symbol at the exchange
    symbol: String,
    /// The currency used to express the symbol's price.
    currency: Option<String>,
    /// The name of the price update provider.
    updater: Option<String>,
    /// The symbol, as used by the updater.
    updater_symbol: Option<String>,
    /// The symbol, as used in the Ledger journal.
    ledger_symbol: Option<String>,
    /// The symbol, as used at Interactive Brokers.
    ib_symbol: Option<String>,
}

#[allow(unused)]
impl SymbolMetadata {
    fn new() -> Self {
        Self::default()
    }
}

// /// The options for parsing.
// pub struct ParseOptions {
//     /// The first line contains column headers?
//     has_header_column: Option<bool>
// }

/// Read and parse the symbols collection.
pub fn read_symbols(path: &PathBuf) -> anyhow::Result<Vec<SymbolMetadata>> {
    //, options: Option<&ParseOptions>

    // read the file
    let mut rdr = csv::Reader::from_path(path)?;

    // parse
    // .records() = raw
    let collection: Vec<SymbolMetadata> = rdr.deserialize()
        .map(|result| result.expect("deserialized row"))
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
}
