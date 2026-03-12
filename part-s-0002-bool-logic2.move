address my_temp_addr {

    module Daily {

        use std::debug::print;

        fun learn_move() {

            let completed: bool = false;

            let ready: bool = true;

            print(&(completed && ready));
            print(&(completed || ready));
            print(&(!completed));
        }

        #[test]
        fun testing() {

            learn_move();
        }
    }
}

// [debug] false
// [debug] true
// [debug] true
