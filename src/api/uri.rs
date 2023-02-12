// use crate::model::metadata::{
//     DisplayType,
//     AttributeValueOnly,

// }

use crate::repository::hashtable::HashTable;
use dotenv::dotenv;
use std::env;
use pinata_sdk::{ApiError, PinataApi, PinByJson};
use hex::FromHex;
use hex;
use std::convert::TryFrom;
use ethers_signers::{Signer, LocalWallet};
use ethers_core::{k256::ecdsa::SigningKey};
use ethers::utils;
use ethers_core::abi::encode;
use ethers_core::types::{Address, U256};

use actix_web::{
    get, 
    post, 
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use serde::{Serialize, Deserialize};
use derive_more::{Display};
use std::collections::HashMap;

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

#[derive(Deserialize, Debug)]
pub struct SubmitIngridients {
    combination: String,
}

#[derive(Debug, Display)]
pub enum TaskError {
    SignatureFailed,
    WalletFailed,
    MetadataFailed,
    NftTaken,
    Forbidden,
    Conflict
}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::SignatureFailed => StatusCode::FAILED_DEPENDENCY,
            TaskError::WalletFailed => StatusCode::FAILED_DEPENDENCY,
            TaskError::MetadataFailed => StatusCode::FAILED_DEPENDENCY,
            TaskError::NftTaken => StatusCode::METHOD_NOT_ALLOWED,
            TaskError::Forbidden => StatusCode::FORBIDDEN,
            TaskError::Conflict => StatusCode::CONFLICT,
        }
    }
}

#[post("/uri")]
pub async fn create_uri(
    request: Json<SubmitIngridients>
) -> Result<Json<SignedURIResponse>, TaskError> {
    let combination = request.combination.clone();
    println!("ingridents are: {}", combination);

     let mut hashtable: HashTable<String> = HashTable::new(365, "hashtable.bin");
     println!("The Hashtable is: \n{:#?}", hashtable);
     let key = hashtable.insert(combination.clone());
     println!("The Hashtable is: \n{:#?}", hashtable.data);
     println!("The Key is: \n{}", key);

    let response = if key < 365 {

        let ingridients: Vec<String> = combination.split("_").map(|s| s.to_string()).collect();
        let mut attributes = Vec::new();
        for i in 0..ingridients.len() {
            // TODO: Add OFFERS as attributes here

            let attribute = Attribute {
                trait_type: Some(String::from("ingredient")),
                value: format!("{}", ingridients[i]),
            };
            attributes.push(attribute);
        };
        let attributes_opensea = attributes.clone();
        println!("The Attributes are: \n{:#?}", attributes);

        // create the metadata
        let name = format!("NFTea TESTNET");
        let image = format!("https://ipfs.io/ipfs/QmcQrUhV9wk24PUXC72gJL1JnSvshBRZZ4E2EJYJ8643V8/{}.png", key + 1); //is this correct?=> array[i] or i ?? <<<<==== Davood =>>>>>>>>
        let description = format!("Our NFTeas are truly special blend utilising the power of mQuark , they are more than an image,  they are transformed into living, breathing pieces of art, each with its own unique flavour and personality. Infinitely upgradable through this metadata they offer true interoperability - take them anywhere!");
        let origins = Origins {
            template: Template {
                id: format!("1"),
                name: format!("mQuark Beverage"),
                image: format!("https://ipfs.io/ipfs/QmTxpSbXqB5m7PsnEzofMnVTCoyUCJy1i224t2Kv9WZoa4"),
                description: format!("This is a Beverage Template, a simple representation of beverage-typed NFT collections."),
                attributes: Some(vec![
                    AttributeValueOnly {
                        value: format!("Type"),
                    },
                    AttributeValueOnly {
                        value: format!("Temperature"),
                    },
                    AttributeValueOnly {
                        value: format!("Ingredient Type"),
                    },
                    AttributeValueOnly {
                        value: format!("Sweetness Level"),
                    },
                    AttributeValueOnly {
                        value: format!("Size"),
                    },
                    AttributeValueOnly {
                        value: format!("Milk Type"),
                    },
                    AttributeValueOnly {
                        value: format!("Effect"),
                    },
                    AttributeValueOnly {
                        value: format!("Container"),
                    },
                    AttributeValueOnly {
                        value: format!("Rarity"),
                    },
                ]),
            },
            project: Project {
                id: format!("1"),
                name: format!("Flying Fish Tea Co."),
                image: format!("https://cdn.shopify.com/s/files/1/0531/1116/0993/files/green_logo-2-2-2-2-2_140x.jpg?v=1636920599"),
                description: Some(format!("https://www.flyingfishtea.co.uk/")),
            },
            collection: Collection {
                id: format!("1"),
                name: format!("NFTea"),
                description: Some(format!("Once upon a time, in a land where teas were kings, six unique ones lived together in harmony. Black tea, White tea, Green tea, Rooibos tea, Pu-erh tea, and Oolong tea each had their own special qualities and lived in separate tea gardens, content with their daily routines. But one day, they heard whispers of a revolutionary new world, a place where they could become more than just tea - the world of Web3.\nExcited by the prospect of becoming something truly unique, the teas decided to embark on a journey together to discover this magical land. Along the way, they gathered special ingredients to enhance their flavors and make themselves stand out from the rest.\nFinally, they arrived at the entrance to the Web3 world - a sprawling marketplace filled with opportunities and challenges. As they explored this new and exciting place, they discovered that they could use blockchain technology to create unique digital representations of themselves, each with their own special blend of ingredients.\nThe teas worked tirelessly, perfecting their digital creations and blending themselves with the finest ingredients. And soon, they were transformed into living, breathing pieces of art, each with its own unique flavor and personality.\nAs they explored the Web3 world, they encountered other digital creations and formed friendships with them. They learned that they could trade their digital representations with others and that their creations would live forever, becoming a part of Web3's rich history.\nAnd so, the six teas lived happily ever after, continuing to explore the wonders of web3 and sharing their unique creations with the world. They knew that they would never forget their journey and the lessons they had learned along the way."
                                )),
                image: Some(format!("{}", "ipfs://[collection-cid]")),
                variations: String::from("dynamic"),
                attributes,
            },
        };

        let raw_metadata = QuarkCollectionMetadataStandard {
            name,
            image,
            description,
            origins,
            attributes: attributes_opensea,
        };

        // Load env variables
        dotenv().ok();

        // IPFS response
        println!("Hello from loaded ENV variables here!");
        // Get the Pinata API key from the environment variables
        let pinata_api_key = env::var("PINATA_API_KEY").expect("PINATA_API_KEY must be set in the .env file");
        // Get the Pinata secret API key from the environment variables
        let pinata_secret_api_key = env::var("PINATA_SECRET_API_KEY").expect("PINATA_SECRET_API_KEY must be set in the .env file");
        println!("pinata_api_key Pinata ENV is: {}", pinata_api_key);
        println!("pinata_secret_api_key Pinata ENV is: {}", pinata_secret_api_key);
        let pinata_api = match PinataApi::new(pinata_api_key, pinata_secret_api_key) {
            Ok(api) => api,
            Err(e) => {
                return Err(TaskError::MetadataFailed);
            }
        };
        println!("Hello from After setting pinata_api!");
        let result = pinata_api.pin_json(PinByJson::new(raw_metadata.clone())).await;
        println!("Hello from After getting pinata_api response!");
        if let Ok(pinned_object) = result {
            let hash = pinned_object.ipfs_hash;
            let ipfs_uri = format!(
                "ipfs://{}", hash 
            );
            println!("The ipfs_uri is: \n{:#?}", ipfs_uri);

            let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY must be set in the .env file");

            println!("The private_key is: \n{:#?}", private_key);

            let signature = match sign_message(&private_key, &ipfs_uri).await {
                Ok(sig) => sig,
                Err(_) => {
                    return Err(TaskError::SignatureFailed);
                }
            };

            let signature_hex = hex::encode(signature);

            let response_data = SignedURIResponse {
                signature: signature_hex, 
                ipfs_uri,
                metadata: raw_metadata
            };

            println!("The signature is: \n{:#?}", response_data);
            Ok::<Json<SignedURIResponse>, TaskError>(Json(response_data))

        } else {
            return Err(TaskError::MetadataFailed);
        }
    } else {
        return Err(TaskError::NftTaken);
    };

    response

}

// fn load_env() {
    
// }

async fn sign_message(hex_private_key: &str, uri: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let wallet = hex_private_key.trim_start_matches("0x").parse::<LocalWallet>().expect("wallet wasnt created");
    println!("wallet address is: {}", wallet.address());

    let verifier = "0x15b9576fF4a224eD08f2E04c77B169a07B9d9D3B".parse::<Address>().expect("failed to parse verifier address");
    let project_id = U256::from(3u64);
    let template_id = U256::from(1u64);
    let collection_id = U256::from(1u64);

    let hash_data = encode(&[
        ethers_core::abi::Token::Address(verifier),
        ethers_core::abi::Token::Uint(project_id),
        ethers_core::abi::Token::Uint(template_id),
        ethers_core::abi::Token::Uint(collection_id),
        ethers_core::abi::Token::String(uri.to_string()),
        ethers_core::abi::Token::String("0x01".to_string()),
    ]);

    let hashed_data = utils::keccak256(hash_data.as_slice());
    let signature = wallet.sign_message(hashed_data).await.expect("signing the message failed!");
    let signature_hex = format!("0x{}", signature);
    hex::decode(signature_hex.trim_start_matches("0x")).map_err(|e| e.into())
}