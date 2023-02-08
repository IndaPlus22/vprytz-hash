#[derive(Clone)]
pub struct Element {
    pub key: String,
    pub value: String,
}

pub struct HashMap {
    data: Vec<Option<Element>>,
}

impl HashMap {
    pub fn new() -> HashMap {
        // set length of data to 50
        const MAP_SIZE: usize = 100;
        HashMap {
            data: vec![None; MAP_SIZE],
        }
    }

    // TODO: implement collision resolution

    fn hash(&self, key: String) -> usize {
        // calculate integer sum of each character in key, then mod by length of data
        let mut sum = 0;
        for c in key.chars() {
            sum += c as usize;
        }
        sum % self.data.len()
    }

    pub fn insert(&mut self, key: String, value: String) {
        let index = self.hash(key.clone());

        self.data[index] = Some(Element { key, value });
    }

    pub fn delete(&mut self, key: String) {
        let index = self.hash(key.clone());

        self.data[index] = None;
    }

    pub fn get(&self, key: String) -> Option<Element> {
        let index = self.hash(key.clone());

        match self.data[index] {
            Some(ref element) => Some(element.clone()),
            None => None,
        }
    }
}
