extern crate mws;
use chrono::offset::TimeZone;
use chrono::{DateTime, NaiveDate, Utc};
use std::path::PathBuf;
use structopt::StructOpt;

use mws::client::Client;

mod env;

use self::env::Env;

#[derive(Debug, StructOpt)]
#[structopt(name = "mws-cli", about = "MWS CLI.")]
struct Opt {
  #[structopt(long = "env", parse(from_os_str))]
  env: Option<PathBuf>,
  #[structopt(subcommand)]
  cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
  ReportRequest {
    #[structopt(long = "report_type")]
    report_type: String,
    #[structopt(long = "start_date", parse(try_from_str))]
    start_date: Option<NaiveDate>,
    #[structopt(long = "end_date", parse(try_from_str))]
    end_date: Option<NaiveDate>,
    #[structopt(long = "report_options")]
    report_options: Option<String>,
    #[structopt(long = "marketplace")]
    marketplace_id_list: Option<Vec<String>>,
  },
  ReportListRequestByIds {
    #[structopt(long = "id")]
    ids: Vec<String>,
  },
  ReportListRequestByTypes {
    #[structopt(long = "type")]
    types: Vec<String>,
  },
  ReportGet {
    #[structopt(long = "id")]
    id: String,
    #[structopt(long = "out", parse(from_os_str))]
    out: PathBuf,
  },
  EncodingConvJp {
    #[structopt(long = "in", parse(from_os_str))]
    input: PathBuf,
    #[structopt(long = "out", parse(from_os_str))]
    out: PathBuf,
  },
  ProductGetLowestPricedOffersForSKU {
    #[structopt(long = "marketplace")]
    marketplace_id: String,
    #[structopt(long = "seller_sku")]
    seller_sku: String,
    #[structopt(long = "condition")]
    condition: String,
  },
}

fn main() {
  let opt = Opt::from_args();

  let env_path = opt.env.unwrap_or_else(|| PathBuf::from(".env"));

  println!("env path: {:?}", env_path);
  ::dotenv::from_path(&env_path).ok();

  let env = Env::from_env();

  println!("seller id: {}", env.seller_id);
  println!("region id: {}", env.region_id);

  let client = get_client(&env);

  match opt.cmd {
    Command::ReportRequest {
      report_type,
      start_date,
      end_date,
      report_options,
      marketplace_id_list,
    } => {
      use mws::reports::*;
      let res = RequestReport(
        &client,
        RequestReportParameters {
          ReportType: report_type,
          StartDate: start_date.map(get_utc_datetime),
          EndDate: end_date.map(get_utc_datetime),
          ReportOptions: report_options,
          MarketplaceIdList: marketplace_id_list,
        },
      )
      .unwrap();
      println!("{:#?}", res)
    }
    Command::ReportListRequestByIds { ids } => {
      use mws::reports::*;
      let res = GetReportRequestList(
        &client,
        GetReportRequestListParameters {
          ReportRequestIdList: Some(ids),
          ..Default::default()
        },
      )
      .unwrap();

      println!("{:#?}", res)
    }
    Command::ReportListRequestByTypes { types } => {
      use mws::reports::*;
      let res = GetReportRequestList(
        &client,
        GetReportRequestListParameters {
          ReportTypeList: Some(types),
          ..Default::default()
        },
      )
      .unwrap();

      println!("{:#?}", res)
    }
    Command::ReportGet { id, out } => {
      use mws::reports::*;
      let mut out = std::fs::File::create(out).unwrap();
      GetReport(&client, id, &mut out).unwrap();
    }
    Command::EncodingConvJp { input, out } => {
      use encoding_rs::*;
      let bytes = std::fs::read(input).unwrap();
      let (cow, encoding_used, had_errors) = SHIFT_JIS.decode(&bytes);
      if had_errors {
        panic!("decode error.")
      }
      println!("encoding_used: {:?}", encoding_used);
      std::fs::write(out, cow.as_ref()).unwrap();
    }
    Command::ProductGetLowestPricedOffersForSKU {
      marketplace_id,
      seller_sku,
      condition,
    } => {
      use mws::products::*;
      let res = GetLowestPricedOffersForSKU(
        &client,
        GetLowestPricedOffersForSKUParameters {
          MarketplaceId: marketplace_id,
          SellerSKU: seller_sku,
          ItemCondition: ItemCondition::from(&condition as &str),
        },
      )
      .unwrap();
      println!("{:#?}", res)
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
    mws_auth_token: env.auth_token.clone(),
    aws_access_key_id: env.access_key_id.clone(),
    secret_key: env.secret_key.clone(),
  };
  Client::new(opts).unwrap()
}

fn get_utc_datetime(date: NaiveDate) -> DateTime<Utc> {
  Utc.from_utc_date(&date).and_hms(0, 0, 0)
}
