use structopt::StructOpt;

/// Publish command options
#[derive(StructOpt, Debug)]
pub struct Publish {
    /// File to publish
    file: Option<String>,
}
