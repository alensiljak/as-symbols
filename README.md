# as-symbols
A component for managing the list of financial symbols, used by other apps.

The purpose of this package is to read and provide a list of financial symbols together with their related information.

The main intended clients for the library are the [PriceDb](https://github.com/alensiljak/pricedb-rust) and [IBFlex](https://github.com/alensiljak/interactive-brokers-flex-rs) applications/libraries.

Symbols replaces the table in the PriceDb database and serves as the collection of symbols for which PriceDb retrieves and manages prices.
IBFlex uses the symbols table to translate the symbols between IB symbols and the symbols used in Ledger journal.
