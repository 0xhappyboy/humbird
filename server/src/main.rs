use clap::Parser;
use cli::cli::Cli;
use humbird::{
    core::server::Server,
    protocol::http::{Request, Response},
    register_router_plugin,
};

mod cli;

fn test(req: Request, mut res: Response) -> Response {
    res
}

fn main() {
    // cli
    //let cli = Cli::parse();

    register_router_plugin!(
        "/"=>test
    );

    Server::config_run(
        humbird::core::server::NetModel::EventPoll,
        "config-template.toml",
    );
}
