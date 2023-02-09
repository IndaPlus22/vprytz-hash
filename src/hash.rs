#[derive(Clone, Debug)]
pub struct Element {
    pub key: String,
    pub value: String,
}

pub struct HashMap {
    data: Vec<Option<Vec<Element>>>,
}

impl HashMap {
    pub fn new() -> HashMap {
        // set length of data to 50
        const START_SIZE: usize = 100;
        HashMap {
            data: vec![None; START_SIZE],
        }
    }

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
        let list = self.data[index].get_or_insert(Vec::new());

        list.push(Element { key, value });
    }

    pub fn delete(&mut self, key: String) {
        let index = self.hash(key.clone());

        for (i, element) in self.data[index].clone().unwrap().iter().enumerate() {
            if element.key == key {
                self.data[index].as_mut().unwrap().remove(i);
            }
        }
    }

    pub fn get(&self, key: String) -> Option<Element> {
        let index = self.hash(key.clone());
        let list = self.data[index].clone();

        if list.is_none() {
            return None;
        }

        let list = list.unwrap();

        for element in list {
            if element.key == key {
                return Some(element);
            }
        }

        None
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut map = HashMap::new();

        map.insert("name".to_string(), "John".to_string());
        map.insert("age".to_string(), "20".to_string());

        assert_eq!(map.get("name".to_string()).unwrap().value, "John");
        assert_eq!(map.get("age".to_string()).unwrap().value, "20");

        map.delete("name".to_string());

        assert_eq!(map.get("name".to_string()).is_none(), true);
        assert_eq!(map.get("age".to_string()).unwrap().value, "20");
    }
}
