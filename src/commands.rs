pub mod done;
pub mod now;
pub mod other;
pub mod schedule;

use crate::commands::now::Now;
use crate::commands::done::Done;
use crate::commands::schedule::Schedule;
use crate::commands::other::Other;

use clap::{Parser, Subcommand};
#[derive(Subcommand, Debug)]
pub enum Commands {
    Now(Now),
    Done(Done),
    Schedule(Schedule),
    Other(Other),
}
