// Create a new function named spawn_spider that takes one argument of type u64 named dna and returns a Spider struct with that dna.

module 0xcafe::spider_nest {
    use std::vector;

    struct SpiderDna has key {
        dna_digits: u64,
        dna_modulus: u64,
    }

    struct Spider has store {
        dna: u64,
    }

    struct SpiderSwarm has key {
        spiders: vector<Spider>,
    }

    fun init_module(cafe_signer: &signer) {
        let dna_digits = 10;
        let dna_modulus = 10 ^ dna_digits;
        move_to(cafe_signer, SpiderDna {
            dna_digits,
            dna_modulus: (dna_modulus as u256),
        });
        move_to(cafe_signer, SpiderSwarm {
            spiders: vector[],
        });
    }

    public fun spawn_spider(dna: u64): Spider {
        Spider {
            dna,
        }
    }
}
