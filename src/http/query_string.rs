use std::{collections::HashMap};

//a=1&b=2&c&d=&e===&d=7&d=abc   -> structure of querystring
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>) //Vec is an heap allocating array
} 

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value>{
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&'){
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('='){
                key = &sub_str[..i];
                val = &sub_str[i + 1 ..];
            }
            data.entry(key)
            //and_modify accepts a closure (anonymous function)
            .and_modify(|existing: &mut Value | match existing  {
                Value::Single(prev_val) => {
                    //dereference original existing pointer to new assigned value
                    *existing = Value::Multiple(vec![prev_val, val]);//vector macro, creates a new vector and adds values
                }
                Value::Multiple(vec) => {
                    vec.push(val); 
                }
            })
            .or_insert(Value::Single(val));
        }
        unimplemented!()
    }
}