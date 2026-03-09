// Add a struct called SpiderDna with a single field called dna_digits of type u64 to the spider_nest module.

module 0xcafe::spider_nest {
    struct SpiderDna has key {
        dna_digits: u64,
    }
}
