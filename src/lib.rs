/*!
The symbols library parses the symbols data file and returns the collection of SymbolMetadata objects.
 */

#[allow(unused)]
#[derive(Debug, Default)]
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

/// Read and parse the symbols collection.
pub fn read_symbols(path: &str) -> Vec<SymbolMetadata> {
    // read file
    let contents = std::fs::read_to_string(path).expect("file read");
    println!("file read: {:?}", contents);

    let mut rdr = csv::Reader::from_path(path).expect("Symbols file read.");

    // parse
    for record in rdr.records() {
        println!("Record: {:?}", record);
    }

    todo!("complete")
}

#[cfg(test)]
mod tests {
    use crate::read_symbols;

    #[test]
    fn read_test_file() {
        let path = "tests/dummy.csv";
        let actual = read_symbols(path);

        assert!(!actual.is_empty());
    }
}
