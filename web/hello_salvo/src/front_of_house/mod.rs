pub mod hosting {
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
}

mod serving {
    fn take_order() {
        serve_order();
        super::hosting::add_to_waitlist(); // parent module
        self::serve_order(); // self module
    }

    fn serve_order() {}

    fn take_payment() {}
}
