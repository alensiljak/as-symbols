/*!
Test as an external client.
*/

use std::path::PathBuf;

use as_symbols::extensions::SymbolsVec;

#[test]
fn test_search() {
    let path = PathBuf::from("tests/dummy.csv");
    let list = as_symbols::read_symbols(&path)
        .expect("symbols loaded");

    assert!(!list.is_empty());

    let actual = list.find_by_full_symbol("CURRENCY:AUD")
        .expect("the record is in the file");
    assert_eq!("EUR", actual.currency.as_ref().expect("The currency is set"));
}