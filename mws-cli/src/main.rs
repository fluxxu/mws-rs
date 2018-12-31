extern crate mws;

use clap::{App, Arg, SubCommand};
use mws::client::Client;

mod env;

use self::env::Env;

fn main() {
  let matches = App::new("MWS CLI")
    .version("0.1")
    .arg(
      Arg::with_name("env")
        .short("e")
        .long("env")
        .value_name("FILE")
        .help("Sets a env file")
        .takes_value(true),
    )
    .subcommand(SubCommand::with_name("test"))
    .subcommand(
      SubCommand::with_name("report").subcommand(
        SubCommand::with_name("get")
          .arg(Arg::with_name("id").index(1))
          .arg(Arg::with_name("output").index(2)),
      ),
    )
    .get_matches();

  let env_name = matches.value_of("env").unwrap_or(".env");

  println!("env file name: {}", env_name);
  ::dotenv::from_filename(env_name).ok();

  let env = Env::from_env();

  println!("seller id: {}", env.seller_id);
  println!("region id: {}", env.region_id);

  let client = get_client(&env);

  if let Some(_) = matches.subcommand_matches("test") {}

  if let Some(matches) = matches.subcommand_matches("report") {
    use mws::reports;

    if let Some(matches) = matches.subcommand_matches("get") {
      use std::fs;
      let report_id = matches.value_of("id").unwrap();
      let output = matches.value_of("output").unwrap();
      let mut out = fs::File::create(output).unwrap();
      reports::GetReport(&client, report_id.to_string(), &mut out).unwrap();
      println!("report {} saved to {}", report_id, output);
    }
  }
}

fn get_client(env: &Env) -> Client {
  use mws::client::ClientOptions;
  use mws::constants;
  let region = constants::get_region(&env.region_id).expect("invalid region id");
  let opts = ClientOptions {
    endpoint: region.endpoint.to_string(),
    seller_id: env.seller_id.clone(),
    mws_auth_token: None,
    aws_access_key_id: env.access_key_id.clone(),
    secret_key: env.secret_key.clone(),
  };
  Client::new(opts).unwrap()
}
