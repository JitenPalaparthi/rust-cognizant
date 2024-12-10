//use demo_package::{greet,greet_msg};
fn main() {
    // greet();
    // greet_msg("Hello Cognizent");
    use  demo_package::greeting::{greet,greet_msg}; // can also call the path inside
        greet();
        greet_msg("Hello Universe");
    demo_package::greeting::greet_msg("Hello Cognizant");

   //demo_package::internal::internal_greet();
}
