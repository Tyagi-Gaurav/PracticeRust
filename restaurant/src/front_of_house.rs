pub mod hosting {
    //Items in a parent module can’t use the private items inside child modules, but items in
    //child modules can use the items in their ancestor modules
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
}

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}
