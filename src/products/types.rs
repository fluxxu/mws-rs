//! Types based on xsd files.

mod default {
  //! [default.xsd](http://g-ecx.images-amazon.com/images/G/01/mwsportal/doc/en_US/products/default.xsd)

  /// DecimalWithUnits
  #[derive(Debug)]
  pub struct DecimalWithUnits {
    pub value: f64,
    pub units: String,
  }

  /// DimensionType
  #[derive(Debug)]
  pub struct DimensionType {
    pub height: DecimalWithUnits,
    pub length: DecimalWithUnits,
    pub width: DecimalWithUnits,
    pub weight: DecimalWithUnits,
  }

  /// Price
  #[derive(Debug)]
  pub struct Price {
    amount: Option<f64>,
    currency_code: Option<String>,
  }

  /// Image
  #[derive(Debug)]
  pub struct Image {
    url: String,
    height: DecimalWithUnits,
    width: DecimalWithUnits,
  }

  /// ItemAttributesType (part of)
  #[derive(Debug)]
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
  #[derive(Debug)]
  pub struct MarketplaceASIN {
    pub marketplace_id: String,
    pub asin: String,
  }

  /// SellerSKUIdentifier
  #[derive(Debug)]
  pub struct SellerSKUIdentifier {
    pub marketplace_id: String,
    pub seller_id: String,
    pub seller_sku: String,
  }

  /// IdentifierType
  #[derive(Debug)]
  pub enum Identifier {
    MarketplaceASIN(MarketplaceASIN),
    SellerSKU(SellerSKUIdentifier),
  }

  /// BaseRelationshipType
  #[derive(Debug)]
  pub struct BaseRelationship {
    pub identifiers: Vec<Identifier>,
  }

  /// BaseRelationship
  #[derive(Debug)]
  pub enum Relationship {
    VariationParent(BaseRelationship),
  }

  /// SalesRankType
  #[derive(Debug)]
  pub struct SalesRank {
    pub product_category_id: String,
    pub rank: i32,
  }

  /// ProductType
  #[derive(Debug)]
  pub struct Product {
    pub identifiers: Vec<Identifier>,
    pub attribute_sets: Vec<Attribute>,
    pub relationships: Vec<Relationship>,
    pub sales_rankings: Vec<SalesRank>,
  }

}
