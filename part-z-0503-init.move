// Write the init_module function to create and store the SpiderDna resource at the address 0xcafe with dna_digits set to 10.

module 0xcafe::spider_nest {
    struct SpiderDna has key {
        dna_digits: u64,
    }

    fun init_module(cafe_signer: &signer) {
        move_to(cafe_signer, SpiderDna {
            dna_digits: 10,
        });
    }
}
