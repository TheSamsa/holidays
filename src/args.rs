use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "holiday")]
pub struct Arguments {
    #[structopt(short, long = "--locale")]
    pub locale: String,
}
