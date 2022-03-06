// pub mod routes {
//     pub mod home;
//     pub mod login;
//     // pub mod newsletters;
// }
// pub mod startup;

// use crate::startup::run;

// // #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
// fn main() {
//     if let Err(e) = run() {
//         println!("run error {:#?}", e);
//     }
// }

///////////////////////////////

#[cfg(test)]
mod tests {
    // Import the code I want to test
    use super::*;
    // My tests
}

////////////////////
use session_based_authentication::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run()?.await
}
