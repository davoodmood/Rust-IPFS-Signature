use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, Read};
use bincode::{serialize, deserialize};

// HashTable Lib
#[derive(Debug, Serialize, Deserialize)]
pub struct HashTable<T> {
    pub data: Vec<Option<T>>,
    capacity: usize,
    file_name: String,
}

impl<T: Hash + std::clone::Clone + std::cmp::PartialEq + Serialize + for<'a> Deserialize<'a>> HashTable<T> {
    pub fn new(capacity: usize, file_name: &str) -> Self {
        let mut data = vec![None; capacity];
        if let Ok(file) = File::open(file_name) {
            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer).unwrap();
            data = deserialize(&buffer).unwrap();
        }
        Self {
            data,
            capacity,
            file_name: file_name.to_string(),
        }
    }

    pub fn insert(&mut self, item: T) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        item.hash(&mut hasher);
        let mut position = hasher.finish() as usize % self.capacity;
        let mut load = 1;

        while self.data[position].is_some() {
            if load > self.capacity {
                return usize::MAX;
            }

            if let Some(ref existing) = self.data[position] {
                if *existing == item {
                    return usize::MAX;
                }
            }

            position = (position + 1) % self.capacity;
            load += 1;
        }

        self.data[position] = Some(item);
        self.save();
        position
    }

    pub fn search(&self, item: &T) -> bool {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        item.hash(&mut hasher);
        let mut position = hasher.finish() as usize % self.capacity;

        while let Some(ref existing) = self.data[position] {
            if *existing == *item {
                return true;
            }

            position = (position + 1) % self.capacity;
        }

        false
    }

    pub fn delete(&mut self, item: &T) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        item.hash(&mut hasher);
        let mut position = hasher.finish() as usize % self.capacity;

        while let Some(ref existing) = self.data[position] {
            if *existing == *item {
                self.data[position] = None;
                self.save();
                return;
            }

            position = (position + 1) % self.capacity;
        }
    }

    pub fn fill(&mut self, items: Vec<T>) {
        for item in items {
            self.fill_one(item);
        }
        self.save();
    }

    fn fill_one(&mut self, item: T) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        item.hash(&mut hasher);
        let mut position = hasher.finish() as usize % self.capacity;
        let mut load = 1;
    
        while self.data[position].is_some() {
            if load > self.capacity {
                return;
            }
    
            position = (position + 1) % self.capacity;
            load += 1;
        }
    
        self.data[position] = Some(item);
    }

    fn save(&self) {
        let serialized = serialize(&self.data).unwrap();
        let file = File::create(&self.file_name).unwrap();
        let mut writer = BufWriter::new(file);
        writer.write(&serialized).unwrap();
    }
}