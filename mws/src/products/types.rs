mod default {
  //! [default.xsd](http://g-ecx.images-amazon.com/images/G/01/mwsportal/doc/en_US/products/default.xsd)

  use xmlhelper::decode;

  /// DecimalWithUnits
  #[derive(Debug, Default, Serialize)]
  pub struct DecimalWithUnits {
    pub value: f64,
    pub units: String,
  }

  /// DimensionType
  #[derive(Debug, Serialize)]
  pub struct DimensionType {
    pub height: DecimalWithUnits,
    pub length: DecimalWithUnits,
    pub width: DecimalWithUnits,
    pub weight: DecimalWithUnits,
  }

  /// Price
  #[derive(Debug, Serialize)]
  pub struct Price {
    amount: Option<f64>,
    currency_code: Option<String>,
  }

  /// Image
  #[derive(Debug, Serialize)]
  pub struct Image {
    url: String,
    height: DecimalWithUnits,
    width: DecimalWithUnits,
  }

  /// ItemAttributesType (part of)
  #[derive(Debug, Serialize)]
  pub enum Attribute {
    Binding(String),
    Brand(String),
    Color(String),
    Feature(String),
    ItemDimensions(DimensionType),
    Label(String),
    Price(Price),
    Manufacturer(String),
    PackageDimensions(DimensionType),
    PartNumber(String),
    ProductGroup(String),
    ProductTypeName(String),
    Publisher(String),
    SmallImage(Image),
    Studio(String),
    Title(String),
    Warranty(String),
  }
}

pub mod product {
  //! [ProductsAPI_Response.xsd](http://g-ecx.images-amazon.com/images/G/01/mwsportal/doc/en_US/products/ProductsAPI_Response.xsd)

  pub use super::default::*;
  //use xml::decode::{XmlDecode, XmlDecoderContext, XmlDecodeEvent, Result as XmlDecodeResult};

  str_enum! {
    pub enum Id {
      ASIN,
      GCID,
      SellerSKU,
      UPC,
      EAN,
      ISBN,
      JAN,
    }
  }

  /// MarketplaceASINType
  #[derive(Debug, Serialize)]
  pub struct MarketplaceASIN {
    pub marketplace_id: String,
    pub asin: String,
  }

  /// SellerSKUIdentifier
  #[derive(Debug, Serialize)]
  pub struct SellerSKUIdentifier {
    pub marketplace_id: String,
    pub seller_id: String,
    pub seller_sku: String,
  }

  /// IdentifierType
  #[derive(Debug, Serialize)]
  pub enum Identifier {
    MarketplaceASIN(MarketplaceASIN),
    SellerSKU(SellerSKUIdentifier),
  }

  /// BaseRelationshipType
  #[derive(Debug, Serialize)]
  pub struct BaseRelationship {
    pub identifiers: Vec<Identifier>,
  }

  /// BaseRelationship
  #[derive(Debug, Serialize)]
  pub enum Relationship {
    VariationParent(BaseRelationship),
  }

  /// SalesRankType
  #[derive(Debug, Serialize)]
  pub struct SalesRank {
    pub product_category_id: String,
    pub rank: i32,
  }

  /// ProductType
  #[derive(Debug, Serialize)]
  pub struct Product {
    pub identifiers: Vec<Identifier>,
    pub attribute_sets: Vec<Attribute>,
    pub relationships: Vec<Relationship>,
    pub sales_rankings: Vec<SalesRank>,
  }
}
