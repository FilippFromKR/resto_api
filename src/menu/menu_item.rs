use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, write};
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

pub trait Item: IntoIterator + Display {
    fn walk_all<F>(&self, f: F)
        where Self: Sized,
              F: FnMut(&MenuItem);
    fn deep_walk<F>(&self, f: Arc<Mutex<F>>)
        where Self: Sized,
              F: FnMut(&MenuItem);
}


#[derive(Debug, Deserialize, Serialize)]
pub struct MenuItem {
    pub name: String,
    pub description: String,
    pub items: Vec<MenuItem>,
    pub params: Option<HashMap<String, String>>,

}


impl MenuItem {
    pub fn iter(&self) -> impl Iterator<Item = &MenuItem> + '_ {
        self.items.iter()
    }
}

impl Display for MenuItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        /// todo: add str_builder or macro
        // write!(f, "{}", result)

    }
}

impl IntoIterator for MenuItem {
    type Item = MenuItem;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl Item for MenuItem {
    fn walk_all<F>(&self, f: F) where Self: Sized, F: FnMut(&MenuItem) {
        let f = Arc::new(Mutex::new(f));

        if !self.items.is_empty() {
            self.deep_walk(f.clone());
        }
        let borrowed: &Mutex<F> = f.borrow();
        let mut guard = borrowed.lock().unwrap();
        guard(&self);
    }

    fn deep_walk<F>(&self, f: Arc<Mutex<F>>) where Self: Sized, F: FnMut(&MenuItem) {
        for item in self.items.iter() {
            if !item.items.is_empty() {
                item.deep_walk(f.clone())
            }
            let borrowed: &Mutex<F> = f.borrow();
            let mut guard = borrowed.lock().unwrap();
            guard(&item);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::any::{Any, TypeId};

    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use super::*;

    #[test]
    fn it_works() {
        let json = serde_json::json!(
            {
                "name": "Layer 1 ",
                "description": "We are working since 2015, year and... ",
                "items": [
                    {
                    "name": "layer 2 from layer 1",
                    "description": "We are working since 2015, year and... ",
                    "items": [
                                {
                                    "name": "layer 3 from layer 1 ",
                                    "description": "We are working since 2015, year and... ",
                                        "items": [
                                    {
                                            "name": "layer 4 from layer 1",
                                    "description": "We are working since 2015, year and... ",
                                        "items": [
                                        {
                                            "name": "layer 5 from layer 1",
                                            "description": "We are working since 2015, year and... ",
                                                "items": [],
                                                "params":{ "key": "value" }
                                                        }],
                                            "params":{ "key": "value" }
                                                }],
                                    "params":{ "key": "value" }
                                }
                            ],
                "params":{ "key": "value" }
            },
                {
                    "name": "layer 1.2 ",
                "description": "We are working since 2015, year and... ",
                "items": [],
                "params":{ "key": "value" }
                }
            ],
                "params":{ "key": "value" }

        });
        let result = serde_json::from_value::<MenuItem>(json).unwrap();
        let f = |obj: &MenuItem| {
            println!("{}", obj.name);
        };
        result.walk_all(f);


        let alco_menu = MenuItem {
            name: "alco".to_string(),
            description: "bar menu".to_string(),
            items: vec![*Box::new(MenuItem {
                name: "red vine".to_string(),
                description: "caberne".to_string(),
                items: vec![],
                params: Some(HashMap::new()),
            })],
            params: None,
        };
        let meal = MenuItem {
            name: "meal".to_string(),
            description: "meal menu".to_string(),
            items: vec![*Box::new(MenuItem {
                name: "Steak".to_string(),
                description: "pig steak".to_string(),
                items: vec![],
                params: Some(HashMap::new()),
            })],
            params: None,
        };

        let menu = MenuItem {
            name: "Root".to_string(),
            description: "Caffe descreption".to_string(),
            items: vec![*Box::new(alco_menu), *Box::new(meal)],
            params: None,
        };
    }
}