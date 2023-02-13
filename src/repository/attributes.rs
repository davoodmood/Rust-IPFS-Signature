use rand::prelude::*;
use crate::model::metadata::Attribute;

// pub fn populate_attributes(vec: &mut Vec<Attribute>) {
//     let mut rng = rand::thread_rng();
//     let mut used_indices = vec![false; vec.len()];
//     for i in 0..200 {
//         let mut index = rng.gen_range(0, vec.len() as usize);
//         while used_indices[index] {
//             index = rng.gen_range(0, vec.len());
//         }
//         vec[index] = Attribute {
//             trait_type: Some("Flying Fish Tea Discount".to_string()),
//             value: "20%".to_string(),
//         };
//         used_indices[index] = true;
//     }

//     for i in 0..20 {
//         let mut index = rng.gen_range(0, vec.len() as usize);
//         while used_indices[index] {
//             index = rng.gen_range(0, vec.len());
//         }
//         vec[index] = Attribute {
//             trait_type: Some("Audit One Discount".to_string()),
//             value: "10%".to_string(),
//         };
//         used_indices[index] = true;
//     }

//     for i in 0..100 {
//         let mut index = rng.gen_range(0, vec.len() as usize);
//         while used_indices[index] {
//             index = rng.gen_range(0, vec.len());
//         }
//         vec[index] = Attribute {
//             trait_type: Some("soonami Event Priority List".to_string()),
//             value: "true".to_string(),
//         };
//         used_indices[index] = true;
//     }
// }

// pub fn populate_attributes(vec: &mut Vec<Attribute>) {
//     let mut used_indices = vec![false; vec.len()];
//     let mut remaining_indices = (0..vec.len()).collect::<Vec<usize>>();

//     let attributes = [
//         (200, Attribute {
//             trait_type: Some("Flying Fish Tea Discount".to_string()),
//             value: "20%".to_string(),
//         }),
//         (20, Attribute {
//             trait_type: Some("Audit One Discount".to_string()),
//             value: "10%".to_string(),
//         }),
//         (100, Attribute {
//             trait_type: Some("soonami Event Priority List".to_string()),
//             value: "true".to_string(),
//         }),
//     ];

//     for (count, attr) in attributes.iter() {
//         for _ in 0..*count {
//             if let Some(index) = remaining_indices.pop() {
//                 vec[index] = *attr;
//                 used_indices[index] = true;
//             }
//         }
//     }

//     shuffle_vector(vec);

// }

// fn shuffle_vector(vec: &mut Vec<Attribute>) {
//     let mut rng = rand::thread_rng();
//     vec.shuffle(&mut rng);
// }

// pub fn populate_attributes(vec: &mut Vec<Attribute>) {
//     let mut rng = rand::thread_rng();

//     let attributes = vec![
//         Attribute {
//             trait_type: Some("Flying Fish Tea Discount".to_string()),
//             value: "-20%".to_string(),
//         },
//         Attribute {
//             trait_type: Some("Audit One Discount".to_string()),
//             value: "-10%".to_string(),
//         },
//         Attribute {
//             trait_type: Some("soonami Event Priority List".to_string()),
//             value: "true".to_string(),
//         },
//     ];

//     let total_attributes = attributes.len();
//     let max_count = vec.len().min(total_attributes);

//     vec[..max_count].clone_from_slice(&attributes[..max_count]);
//     vec.shuffle(&mut rng);
// }

pub fn populate_attributes(vec: &mut Vec<Attribute>) -> Vec<Attribute> {
    let mut rng = rand::thread_rng();

    // let mut count = 0;
    for i in 0..200 {
        let attribute = Attribute {
            trait_type: Some("Flying Fish Tea Discount".to_string()),
            value: "20%".to_string(),
        };
        vec[i] = attribute;
    }
    for i in 200..220 {
        let attribute = Attribute {
            trait_type: Some("Audit One Discount".to_string()),
            value: "10%".to_string(),
        };
        vec[i] = attribute;
    }
    for i in 220..320 {
        let attribute = Attribute {
            trait_type: Some("soonami Event Priority List".to_string()),
            value: "true".to_string(),
        };
        vec[i] = attribute;
    }
    for i in 320..356 {
        let attribute = Attribute {
            trait_type: Some("".to_string()),
            value: "".to_string(),
        };
        vec[i] = attribute;
    }
    
    vec.shuffle(&mut rng);
    vec.to_vec()
}


