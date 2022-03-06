pub mod routes {
    pub mod home;
    pub mod login;
}
pub mod startup;

use crate::startup::run;

// #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
fn main() {
    if let Err(e) = run() {
        println!("run error {:#?}", e);
    }
}
