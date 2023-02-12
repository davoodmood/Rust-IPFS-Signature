use serde::{Serialize, Deserialize};
use strum_macros::{EnumString, Display};

#[derive(Serialize, Deserialize, Debug)]
struct SignedURIResponse {
  signature: String,
  ipfs_uri: String,
  metadata: QuarkCollectionMetadataStandard,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct QuarkCollectionMetadataStandard {
  name: String,
  image: String,
  description: String,
  origins: Origins,
  attributes: Vec<Attribute>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Origins {
  template: Template,
  project: Project,
  collection: Collection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Template {
  id: String,
  name: String,
  image: String,
  description: String,
  attributes: Option<Vec<AttributeValueOnly>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Project {
  id: String,
  name: String,
  image: String,
  description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Collection {
  id: String,
  name: String,
  description: Option<String>,
  image: Option<String>,
  variations: String,
  attributes: Vec<Attribute>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Variations {
  Dynamic,
  Static(u32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Attribute {
  trait_type: Option<String>, // ingrident
  value: String, // e.g. blacktea
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AttributeValueOnly {
      value: String, // e.g. blacktea
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum DisplayType {
  BoostPercentage,
  BoostNumber,
  Number,
  Date,
}