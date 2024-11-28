use clap::Parser;

#[derive(Parser)]
pub struct Config {
    #[clap(long, env, default_value_t = 7111)]
    pub http_port: u16,
}
