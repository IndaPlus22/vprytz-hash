pub struct HashMap {}

impl HashMap {
    pub fn new() -> HashMap {
        HashMap {}
    }

    // TODO: implement hash function
    // TODO: implement get, insert and delete
    // TODO: implement collision resolution

    pub fn insert(&self, key: String, value: String) {
        println!("Inserting key: {}, value: {}", key, value);
    }

    pub fn delete(&self, key: String) {
        println!("Deleting key: {}", key);
    }

    pub fn get(&self, key: String) {
        println!("Getting key: {}", key);
    }
}
