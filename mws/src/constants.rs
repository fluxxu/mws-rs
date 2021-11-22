pub struct AmazonRegion {
  pub id: &'static str,
  pub name: &'static str,
  pub endpoint: &'static str,
  pub marketplace_id_list: Vec<&'static str>,
  pub marketplaces: Vec<&'static AmazonMarketplace>,
}

pub struct AmazonMarketplace {
  pub id: &'static str,
  pub name: &'static str,
  pub region_id: &'static str,
  pub country_id: &'static str,
  pub fulfillment_countries: &'static [&'static str],
}

pub const REGION_ID_NA: &'static str = "na";
pub const REGION_ID_EU: &'static str = "eu";
pub const REGION_ID_IN: &'static str = "in";
pub const REGION_ID_CN: &'static str = "cn";
pub const REGION_ID_FE: &'static str = "fe";

pub const MARKETPLACE_ID_BR: &'static str = "A2Q3Y263D00KWC";
pub const MARKETPLACE_ID_CA: &'static str = "A2EUQ1WTGCTBG2";
pub const MARKETPLACE_ID_US: &'static str = "ATVPDKIKX0DER";
pub const MARKETPLACE_ID_MX: &'static str = "A1AM78C64UM0Y8";

pub const MARKETPLACE_ID_AE: &'static str = "A2VIGQ35RCS4UG";
pub const MARKETPLACE_ID_DE: &'static str = "A1PA6795UKMFR9";
pub const MARKETPLACE_ID_EG: &'static str = "ARBP9OOSHTCHU";
pub const MARKETPLACE_ID_ES: &'static str = "A1RKKUPIHCS9HS";
pub const MARKETPLACE_ID_FR: &'static str = "A13V1IB3VIYZZH";
pub const MARKETPLACE_ID_GB: &'static str = "A1F83G8C2ARO7P";
pub const MARKETPLACE_ID_IN: &'static str = "A21TJRUUN4KGV";
pub const MARKETPLACE_ID_IT: &'static str = "APJ6JRA9NG5V4";
pub const MARKETPLACE_ID_NL: &'static str = "A1805IZSGTT6HS";
pub const MARKETPLACE_ID_PL: &'static str = "A1C3SOZRARQ6R3";
pub const MARKETPLACE_ID_SA: &'static str = "A17E79C6D8DWNP";
pub const MARKETPLACE_ID_SE: &'static str = "A2NODRKZP88ZB9";
pub const MARKETPLACE_ID_TR: &'static str = "A33AVAJ2PDY3EV";

pub const MARKETPLACE_ID_JP: &'static str = "A1VC38T7YXB528";
pub const MARKETPLACE_ID_AU: &'static str = "A39IBJ37TRP1C6";
pub const MARKETPLACE_ID_SG: &'static str = "A19VAU5U5O7RUS";

pub const MARKETPLACE_ID_CN: &'static str = "AAHKV2X7AFYLW";

lazy_static! {
  pub static ref MARKETPLACES: Vec<AmazonMarketplace> = {
    macro_rules! item {
      ($id:expr, $region_id:expr, $name:expr, $country_id:expr) => {
        AmazonMarketplace {
          id: $id,
          name: $name,
          region_id: $region_id,
          country_id: $country_id,
          fulfillment_countries: &[$country_id]
        }
      };
      ($id:expr, $region_id:expr, $name:expr, $country_id:expr, [$($fulfillment_country:expr),*]) => {
        AmazonMarketplace {
          id: $id,
          name: $name,
          region_id: $region_id,
          country_id: $country_id,
          fulfillment_countries: &[$($fulfillment_country),*],
        }
      };
    }
    let mut items = vec![];
    items.push(item!(MARKETPLACE_ID_CA, "na", "Brazil", "BR"));
    items.push(item!(MARKETPLACE_ID_CA, "na", "Canada", "CA"));
    items.push(item!(MARKETPLACE_ID_MX, "na", "Mexico", "MX"));
    items.push(item!(MARKETPLACE_ID_US, "na", "USA", "US"));

    items.push(item!(MARKETPLACE_ID_AE, "eu", "United Arab Emirates (U.A.E.)", "AE"));
    items.push(item!(MARKETPLACE_ID_DE, "eu", "Germany", "DE", ["DE", "PL", "AT"]));
    items.push(item!(MARKETPLACE_ID_EG, "eu", "Egypt", "EG"));
    items.push(item!(MARKETPLACE_ID_ES, "eu", "Spain", "ES"));
    items.push(item!(MARKETPLACE_ID_FR, "eu", "France", "FR"));
    items.push(item!(MARKETPLACE_ID_GB, "eu", "United Kingdom", "GB"));
    items.push(item!(MARKETPLACE_ID_IN, "in", "India", "IN"));
    items.push(item!(MARKETPLACE_ID_IT, "eu", "Italy", "IT"));
    items.push(item!(MARKETPLACE_ID_NL, "eu", "Netherlands", "NL"));
    items.push(item!(MARKETPLACE_ID_PL, "eu", "Poland", "PL"));
    items.push(item!(MARKETPLACE_ID_SA, "eu", "Saudi Arabia", "SA"));
    items.push(item!(MARKETPLACE_ID_SE, "eu", "Sweden", "SE"));
    items.push(item!(MARKETPLACE_ID_TR, "eu", "Turkey", "TR"));


    items.push(item!(MARKETPLACE_ID_JP, "fe", "Japan", "JP"));
    items.push(item!(MARKETPLACE_ID_AU, "fe", "Australia", "AU"));
    items.push(item!(MARKETPLACE_ID_SG, "fe", "Singapore", "SG"));

    items.push(item!(MARKETPLACE_ID_CN, "cn", "China", "CN"));

    items
  };
  pub static ref REGIONS: Vec<AmazonRegion> = {
    vec![
      AmazonRegion {
        id: REGION_ID_NA,
        name: "North America (NA)",
        endpoint: "mws.amazonservices.com",
        marketplace_id_list: get_region_marketplace_id_list("na"),
        marketplaces: get_region_marketplace_list("na"),
      },
      AmazonRegion {
        id: REGION_ID_EU,
        name: "Europe (EU)",
        endpoint: "mws-eu.amazonservices.com",
        marketplace_id_list: get_region_marketplace_id_list("eu"),
        marketplaces: get_region_marketplace_list("eu"),
      },
      AmazonRegion {
        id: REGION_ID_IN,
        name: "India (IN)",
        endpoint: "mws.amazonservices.com",
        marketplace_id_list: get_region_marketplace_id_list("in"),
        marketplaces: get_region_marketplace_list("in"),
      },
      AmazonRegion {
        id: REGION_ID_CN,
        name: "China (CN)",
        endpoint: "mws.amazonservices.com.cn",
        marketplace_id_list: get_region_marketplace_id_list("cn"),
        marketplaces: get_region_marketplace_list("cn"),
      },
      AmazonRegion {
        id: REGION_ID_FE,
        name: "Far East (FE)",
        endpoint: "mws.amazonservices.com.au",
        marketplace_id_list: get_region_marketplace_id_list("fe"),
        marketplaces: get_region_marketplace_list("fe"),
      },
    ]
  };
}

pub fn get_marketplace_region(id: &str) -> Option<&str> {
  get_marketplace(id).map(|m| m.region_id)
}

pub fn get_region_marketplace_id_list(region_id: &str) -> Vec<&'static str> {
  MARKETPLACES
    .iter()
    .filter_map(|m| {
      if m.region_id == region_id {
        Some(m.id)
      } else {
        None
      }
    })
    .collect()
}

pub fn get_region_marketplace_list(region_id: &str) -> Vec<&'static AmazonMarketplace> {
  MARKETPLACES
    .iter()
    .filter_map(|m| {
      if m.region_id == region_id {
        Some(m)
      } else {
        None
      }
    })
    .collect()
}

pub fn get_region(id: &str) -> Option<&'static AmazonRegion> {
  REGIONS.iter().find(|r| r.id == id)
}

pub fn get_marketplace(id: &str) -> Option<&'static AmazonMarketplace> {
  MARKETPLACES.iter().find(|r| r.id == id)
}

impl AmazonMarketplace {
  pub fn resolve_state_code(&self, country_id: &str, state: &str) -> Option<String> {
    match self.id {
      MARKETPLACE_ID_US if country_id == "US" => resolve_usa_state_code(state),
      MARKETPLACE_ID_AU if country_id == "AU" => resolve_au_state_code(state),
      _ => Some(state.to_string()),
    }
  }
}

const US_STATES: &'static [(&'static str, &'static str)] = &[
  ("AK", "alaska"),
  ("AL", "alabama"),
  ("AP", "apo/fpo: asia, pacific"),
  ("AR", "arkansas"),
  ("AZ", "arizona"),
  ("CA", "california"),
  ("CO", "colorado"),
  ("CT", "connecticut"),
  ("DC", "district of columbia"),
  ("DE", "delaware"),
  ("FL", "florida"),
  ("GA", "georgia"),
  ("HI", "hawaii"),
  ("IA", "iowa"),
  ("ID", "idaho"),
  ("IL", "illinois"),
  ("IN", "indiana"),
  ("KS", "kansas"),
  ("KY", "kentucky"),
  ("LA", "louisiana"),
  ("MA", "massachusetts"),
  ("MD", "maryland"),
  ("ME", "maine"),
  ("MI", "michigan"),
  ("MN", "minnesota"),
  ("MO", "missouri"),
  ("MS", "mississippi"),
  ("MT", "montana"),
  ("NC", "north carolina"),
  ("ND", "north dakota"),
  ("NE", "nebraska"),
  ("NH", "new hampshire"),
  ("NJ", "new jersey"),
  ("NM", "new mexico"),
  ("NV", "nevada"),
  ("NY", "new york"),
  ("OH", "ohio"),
  ("OK", "oklahoma"),
  ("OR", "oregon"),
  ("PA", "pennsylvania"),
  ("RI", "rhode island"),
  ("SC", "south carolina"),
  ("SD", "south dakota"),
  ("TN", "tennessee"),
  ("TX", "texas"),
  ("UT", "utah"),
  ("VA", "virginia"),
  ("VT", "vermont"),
  ("WA", "washington"),
  ("WI", "wisconsin"),
  ("WV", "west virginia"),
  ("WY", "wyoming"),
];

fn resolve_usa_state_code(state: &str) -> Option<String> {
  let v = normalize(state);
  if v.len() == 2 {
    let id_match = US_STATES.iter().find(|(id, _)| id == &v.to_uppercase());
    if let Some((id, _)) = id_match {
      return Some(id.to_string());
    }
  }

  let text_match = US_STATES.iter().find(|(_, text)| text == &v);
  if let Some((id, _)) = text_match {
    return Some(id.to_string());
  }

  None
}

#[test]
fn test_resolve_usa_state_code() {
  [
    "Al",
    "AL",
    "Alabama",
    "ALABAMA",
    "Alaska",
    "AR",
    "Arizona",
    "ARIZONA",
    "Arkansas",
    "az",
    "Az",
    "AZ",
    "ca",
    "Ca",
    "Ca.",
    "CA",
    "California",
    "CALIFORNIA",
    "Co",
    "CO",
    "Colorado",
    "COLORADO",
    "Connecticut",
    "CONNECTICUT",
    "Ct",
    "CT",
    "DC",
    "DE",
    "Delaware",
    "DELAWARE",
    "District of Columbia",
    "DISTRICT OF COLUMBIA",
    "fl",
    "Fl",
    "FL",
    "florida",
    "Florida",
    "FLORIDA",
    "Ga",
    "GA",
    "Georgia",
    "GEORGIA",
    "Hawaii",
    "HI",
    "IA",
    "ID",
    "Idaho",
    "il",
    "Il",
    "IL",
    "Illinois",
    "ILLINOIS",
    "IN",
    "Indiana",
    "INDIANA",
    "Iowa",
    "IOWA",
    "Kansas",
    "KANSAS",
    "Kentucky",
    "KENTUCKY",
    "ks",
    "KS",
    "Ky",
    "KY",
    "LA",
    "Louisiana",
    "ma",
    "Ma",
    "MA",
    "Maine",
    "MAINE",
    "Maryland",
    "MARYLAND",
    "massachusetts",
    "Massachusetts",
    "MASSACHUSETTS",
    "Md",
    "MD",
    "Me",
    "ME",
    "mi",
    "MI",
    "Michigan",
    "MICHIGAN",
    "Minnesota",
    "MINNESOTA",
    "Mississippi",
    "MISSISSIPPI",
    "Missouri",
    "MISSOURI",
    "mn",
    "MN",
    "mo",
    "Mo",
    "MO",
    "Montana",
    "MONTANA",
    "MS",
    "MT",
    "nc",
    "Nc",
    "NC",
    "ND",
    "NE",
    "Nebraska",
    "Nevada",
    "NEVADA",
    "New Hampshire",
    "New Jersey",
    "NEW JERSEY",
    "New Mexico",
    "NEW MEXICO",
    "new york",
    "New York",
    "NEW YORK",
    "NH",
    "nj",
    "Nj",
    "NJ",
    "N.J.",
    "NM",
    "North Carolina",
    "North Dakota",
    "NV",
    "ny",
    "Ny",
    "NY",
    "N.Y.",
    "OH",
    "ohio",
    "Ohio",
    "OHIO",
    "OK",
    "Oklahoma",
    "OKLAHOMA",
    "OR",
    "Oregon",
    "OREGON",
    "pa",
    "Pa",
    "PA",
    "Pennsylvania",
    "PENNSYLVANIA",
    "Rhode Island",
    "RHODE ISLAND",
    "RI",
    "SC",
    "SD",
    "South Carolina",
    "SOUTH CAROLINA",
    "Tennessee",
    "texas",
    "Texas",
    "TEXAS",
    "tn",
    "TN",
    "Tx",
    "TX",
    "TX ",
    "UT",
    "Utah",
    "UTAH",
    "Va",
    "VA",
    "Vermont",
    "VERMONT",
    "Virginia",
    "VIRGINIA",
    "VT",
    "wa",
    "Wa",
    "WA",
    "WA ",
    "Washington",
    "WASHINGTON",
    "West Virginia",
    "WEST VIRGINIA",
    "wi",
    "WI",
    "Wisconsin",
    "WISCONSIN",
    "WV",
    "WY",
    "Wyoming",
    "WYOMING",
  ]
  .iter()
  .for_each(|v| {
    assert!(
      resolve_usa_state_code(v).is_some(),
      "{}: {}",
      v,
      normalize(v)
    )
  })
}

const AU_STATES: &'static [(&'static str, &'static str)] = &[
  ("ACT", "australian capital territory"),
  ("NSW", "new south wales"),
  ("NT", "northern territory"),
  ("QLD", "queensland"),
  ("SA", "south australia"),
  ("TAS", "tasmania"),
  ("VIC", "victoria"),
  ("WA", "western australia"),
];

fn resolve_au_state_code(state: &str) -> Option<String> {
  let v = normalize(state);
  if v.len() == 2 || v.len() == 3 {
    let id_match = AU_STATES.iter().find(|(id, _)| id == &v.to_uppercase());
    if let Some((id, _)) = id_match {
      return Some(id.to_string());
    }
  }

  let text_match = AU_STATES.iter().find(|(_, text)| text == &v);
  if let Some((id, _)) = text_match {
    return Some(id.to_string());
  }

  None
}

#[test]
fn test_resolve_au_state_code() {
  [
    "ACT",
    // "Australia",
    "Australian Capital Territory",
    // "Brisbane",
    "New South Wales",
    "Northern Territory",
    "nsw",
    "Nsw",
    "NSW",
    "N.S.W.",
    "NT",
    "Qld",
    "QLD",
    "QLD ",
    "queensland",
    "Queensland",
    "SA",
    "South Australia",
    // "sydney",
    "Tas",
    "TAS",
    "Tasmania",
    "vic",
    "Vic",
    "VIC",
    "victoria",
    "Victoria",
    "VICTORIA",
    "w.a.",
    "Wa",
    "WA",
    "W.A.",
    "Western Australia",
    "WESTERN AUSTRALIA",
  ]
  .iter()
  .for_each(|v| {
    assert!(
      resolve_au_state_code(v).is_some(),
      "{}: {}",
      v,
      normalize(v)
    )
  })
}

fn normalize(text: &str) -> String {
  text
    .to_lowercase()
    .chars()
    .filter(|c| c.is_alphabetic() || c.is_whitespace())
    .collect::<String>()
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ")
}
