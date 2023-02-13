use serde::{Serialize, Deserialize};
use strum_macros::{EnumString, Display};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug)]
pub struct SignedURIResponse {
  signature: String,
  ipfs_uri: String,
  metadata: QuarkCollectionMetadataStandard,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuarkCollectionMetadataStandard {
  name: String,
  image: String,
  description: String,
  origins: Origins,
  attributes: Vec<Attribute>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Origins {
  template: Template,
  project: Project,
  collection: Collection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Template {
  id: String,
  name: String,
  image: String,
  description: String,
  attributes: Option<Vec<AttributeValueOnly>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
  id: String,
  name: String,
  image: String,
  description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Collection {
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
  pub trait_type: Option<String>, // ingrident
  pub value: String, // e.g. blacktea
}

impl Hash for Attribute {
  fn hash<H: Hasher>(&self, state: &mut H) {
      self.trait_type.hash(state);
      self.value.hash(state);
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeValueOnly {
      value: String, // e.g. blacktea
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum DisplayType {
  BoostPercentage,
  BoostNumber,
  Number,
  Date,
}