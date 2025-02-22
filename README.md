
A temporary test program to see how a large txt file (> 15 million lines) can best be handled and processed in Rust.
The data is from a Geonames download of 'Alternate names'

Created to explore use of the csv crate.
Seems to function without problems, takes about 90 seconds to load all data, including the aggregation of language codes for names that are assigned to more than one language.
Non-Latin names excluded by default, can be added via a '-n' flag.
