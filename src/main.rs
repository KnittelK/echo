use clap::Parser;
use echo::Echo;

fn main () {
    Echo::parse().run();
}