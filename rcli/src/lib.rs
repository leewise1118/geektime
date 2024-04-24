pub mod opts;
pub mod process;
pub use opts::{Opts, SubCommand};
pub use process::{csv_convert::process_csv, gen_pass::process_genpasswd};
