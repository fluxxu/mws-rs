#[macro_use]
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

  if let Some(_) = matches.subcommand_matches("test") {
    use mws::tdff::TdffParser;
    use std::io::Cursor;

    let bytes = include_bytes!("../../report.txt") as &[u8];
    let str_value = String::from_utf8_lossy(bytes).to_string();
    let r = Cursor::new(str_value.as_bytes());
    let rows = TdffParser::new(r)
      .unwrap()
      .parse_all::<GetMerchantListingsAllDataRow>()
      .unwrap();
    println!("{:#?}", rows)
  }

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

#[derive(Debug, Default, FromTdffRow)]
struct GetMerchantListingsAllDataRow {
  #[from_tdff_row(key = "item-name")]
  pub item_name: String,
  #[from_tdff_row(key = "item-description")]
  pub item_description: String,
  #[from_tdff_row(key = "listing-id")]
  pub listing_id: String,
  #[from_tdff_row(key = "seller-sku")]
  pub seller_sku: String,
  pub price: String,
  pub quantity: String,
  #[from_tdff_row(key = "open-date")]
  pub open_date: String,
  #[from_tdff_row(key = "image-url")]
  pub image_url: String,
  #[from_tdff_row(key = "item-is-marketplace")]
  pub item_is_marketplace: String,
  #[from_tdff_row(key = "product-id-type")]
  pub product_id_type: String,
  #[from_tdff_row(key = "zshop-shipping-fee")]
  pub zshop_shipping_fee: String,
  #[from_tdff_row(key = "item-note")]
  pub item_note: String,
  #[from_tdff_row(key = "item-condition")]
  pub item_condition: String,
  #[from_tdff_row(key = "zshop-category1")]
  pub zshop_category1: String,
  #[from_tdff_row(key = "zshop-browse-path")]
  pub zshop_browse_path: String,
  #[from_tdff_row(key = "zshop-storefront-feature")]
  pub zshop_storefront_feature: String,
  pub asin1: String,
  pub asin2: String,
  pub asin3: String,
  #[from_tdff_row(key = "will-ship-internationally")]
  pub will_ship_internationally: String,
  #[from_tdff_row(key = "expedited-shipping")]
  pub expedited_shipping: String,
  #[from_tdff_row(key = "zshop-boldface")]
  pub zshop_boldface: String,
  #[from_tdff_row(key = "product-id")]
  pub product_id: String,
  #[from_tdff_row(key = "bid-for-featured-placement")]
  pub bid_for_featured_placement: String,
  #[from_tdff_row(key = "add-delete")]
  pub add_delete: String,
  #[from_tdff_row(key = "pending-quantity")]
  pub pending_quantity: String,
  #[from_tdff_row(key = "fulfillment-channel")]
  pub fulfillment_channel: String,
  #[from_tdff_row(key = "merchant-shipping-group")]
  pub merchant_shipping_group: String,
  pub status: String,
}
