# as-symbols
A library for managing the list of financial symbols and related information

# Purpose

The purpose of this package is to read and provide a list of financial symbols together with their related information.

The main intended clients for the library are the [PriceDb](https://github.com/alensiljak/pricedb-rust) and [IBFlex](https://github.com/alensiljak/interactive-brokers-flex-rs) applications/libraries.

Symbols replaces the table in the PriceDb database and serves as the collection of symbols for which PriceDb retrieves and manages prices.
IBFlex uses the symbols table to translate the symbols between IB symbols and the symbols used in Ledger journal.

# Use

TODO

# Format

The library reads the file containing the symbols data. Based on tests, the CSV format seems the simplest and the most-performant for this purpose.

The columns in the file are:

TODO
