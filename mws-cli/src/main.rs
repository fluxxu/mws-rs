extern crate mws;
use chrono::offset::TimeZone;
use chrono::{DateTime, NaiveDate, Utc};
use std::path::PathBuf;
use structopt::StructOpt;

use mws::client::{Client, Method};

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
  ProductGetMyPriceForASIN {
    #[structopt(long = "marketplace")]
    marketplace_id: String,
    #[structopt(long = "asin")]
    asins: Vec<String>,
    #[structopt(long = "condition")]
    condition: Option<String>,
  },
  SubmitFeed {
    #[structopt(long = "feed_type")]
    feed_type: String,
    #[structopt(long = "contant_file", parse(from_os_str))]
    content_file: PathBuf,
    #[structopt(long = "marketplace")]
    marketplace_id_list: Option<Vec<String>>,
    #[structopt(long = "content_type")]
    content_type: String,
  },
  ListSubscriptions {
    #[structopt(long = "marketplace")]
    marketplace_id: String,
  },
  ListFinancialEvents {
    #[structopt(long = "posted_after", parse(try_from_str))]
    posted_after: NaiveDate,
    #[structopt(long = "posted_before", parse(try_from_str))]
    posted_before: NaiveDate,
    #[structopt(long = "out", parse(from_os_str))]
    outdir: PathBuf,
  },
  ListOrders {
    #[structopt(long = "marketplace")]
    marketplace_id_list: Vec<String>,
    #[structopt(long = "created_after", parse(try_from_str))]
    created_after: NaiveDate,
    #[structopt(long = "out", parse(from_os_str))]
    outdir: PathBuf,
  }
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
    Command::ProductGetMyPriceForASIN {
      marketplace_id,
      asins,
      condition,
    } => {
      use mws::products::*;
      let res = GetMyPriceForASIN(
        &client,
        GetMyPriceForASINParameters {
          MarketplaceId: marketplace_id,
          ASINList: asins,
          ItemCondition: condition.map(|v| ItemCondition::from(&v as &str)),
        },
      )
      .unwrap();
      println!("{:#?}", res)
    }
    Command::SubmitFeed {
      feed_type,
      content_file,
      marketplace_id_list,
      content_type,
    } => {
      use mws::feeds::*;
      use std::io::Cursor;
      let content = std::fs::read(content_file).unwrap();
      let digest = md5::compute(&content);
      let b64 = base64::encode(&*digest);
      let res = SubmitFeed(
        &client,
        SubmitFeedParameters {
          FeedType: feed_type,
          MarketplaceIdList: marketplace_id_list,
          PurgeAndReplace: None,
        },
        Cursor::new(content),
        b64,
        content_type,
      )
      .unwrap();
      println!("{:#?}", res)
    }
    Command::ListSubscriptions { marketplace_id } => {
      use mws::subscriptions::*;
      let res = ListSubscriptions(&client, marketplace_id).unwrap();
      println!("{:#?}", res)
    }
    Command::ListFinancialEvents {
      posted_after,
      posted_before,
      outdir,
    } => {
      use std::thread::sleep;
      use std::time::Duration;
      let mut page = 1;
      let mut next_token: Option<String> = None;

      let duration = posted_before - posted_after;
      let days = duration.num_days();

      if days < 0 {
        panic!("invalid date range")
      }

      let chunks = ((days as f64) / 180.0).ceil() as i64;
      let ranges: Vec<[NaiveDate; 2]> = (0..chunks).into_iter().map(|chunk_idx| {
        [posted_after + chrono::Duration::days(chunk_idx * 180), std::cmp::min(posted_before, posted_after + chrono::Duration::days((chunk_idx + 1) * 180))]
      }).collect();

      println!("ranges = {:?}", ranges);

      for [posted_after, posted_before] in ranges {
        loop {
          println!("loading page {} ...", page);

          let res = if let Some(next_token) = next_token.take() {
            client
              .request_xml_generic(
                Method::POST,
                "/Finances",
                "2015-05-01",
                "ListFinancialEventsByNextToken",
                vec![
                  (
                    "NextToken".to_string(),
                    next_token,
                  ),
                ],
              )
          } else {
            client
              .request_xml_generic(
                Method::POST,
                "/Finances",
                "2015-05-01",
                "ListFinancialEvents",
                vec![
                  (
                    "PostedAfter".to_string(),
                    get_utc_datetime(posted_after).to_rfc3339(),
                  ),
                  (
                    "PostedBefore".to_string(),
                    get_utc_datetime(posted_before).to_rfc3339(),
                  ),
                ],
              )
          };

          
          let res = match res {
            Ok(res) => res,
            Err(err) => {
              if err.should_try_again() {
                eprintln!("RetriableException: {:?}", err);
                continue;
              } else {
                Err(err).unwrap()
              }
            }
          };

          let filename = format!("financial_events_{}_{}_{}.xml", posted_after, posted_before, page);
          let f = std::fs::File::create(outdir.join(filename)).unwrap();
          res.result_element.write(f).unwrap();

          next_token = res.next_token().clone().map(|v| v.to_string());
          if next_token.is_none() {
            break;
          }

          page += 1;

          sleep(Duration::from_secs(1));
        }
      }
    }
    Command::ListOrders {
      marketplace_id_list,
      created_after,
      outdir
    } => {
      use std::thread::sleep;
      use std::time::Duration;
      use mws::orders::*;

      let mut page = 1;
      let mut next_token = None;

      loop {
        println!("loading page = {}", page);

        let res = if let Some(next_token) = next_token.clone() {
          ListOrdersByNextToken(&client, next_token)
        } else {
          ListOrders(&client, ListOrdersParameters {
            MarketplaceId: marketplace_id_list.clone(),
            CreatedAfter: Some(DateTime::<Utc>::from_utc(created_after.and_hms(0, 0, 0).into(), Utc)),
            MaxResultsPerPage: Some(100),
            ..Default::default()
          })
        };

        match res {
          Ok(res) =>{
            next_token = res.NextToken;
            let orders = res.Orders;
            let mut json_orders = vec![];
            println!("orders = {}", orders.len());

            for order in orders {
              println!("loading items: {} {:?}", order.AmazonOrderId, order.PurchaseDate);

              let mut next_token = None;
              let mut items = vec![];
              loop {
                let res = if let Some(next_token) = next_token.clone() {
                  ListOrderItemsByNextToken(&client, next_token)
                } else {
                  ListOrderItems(&client, order.AmazonOrderId.clone())
                };

                match res {
                  Ok(res) => {
                    next_token = res.NextToken;
                    println!("items = {}", res.OrderItems.len());
                    items.extend(res.OrderItems);
                  }
                  Err(err) => {
                    if err.should_try_again() {
                      sleep(Duration::from_secs(10));
                      continue;
                    } else {
                      Err(err).unwrap()
                    }
                  }
                }

                sleep(Duration::from_secs(1));

                if next_token.is_none() {
                  break;
                }
              }

              json_orders.push(serde_json::json!({
                "order": serde_json::to_value(&order).unwrap(),
                "items": serde_json::to_value(&items).unwrap()
              }));
            }

            let filename = format!("orders_{}.json", page);
            let f = std::fs::File::create(outdir.join(filename)).unwrap();
            serde_json::to_writer(f, &json_orders).unwrap();
          },
          Err(err) => {
            if err.should_try_again() {
              sleep(Duration::from_secs(10));
              continue;
            } else {
              Err(err).unwrap()
            }
          }
        }

        if next_token.is_none() {
          break;
        }

        page += 1;
        sleep(Duration::from_secs(5));
      }
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
