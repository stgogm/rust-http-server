use std::collections::HashMap;

#[derive(Debug)]
pub enum Value<'buf> {
  Multi(Vec<&'buf str>),
  Single(&'buf str),
}

#[derive(Debug)]
pub struct QueryString<'buf> {
  data: HashMap<&'buf str, Value<'buf>>,
}

impl<'buf> QueryString<'buf> {
  pub fn get(&self, key: &str) -> Option<&Value> {
    self.data.get(key)
  }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
  fn from(s: &'buf str) -> Self {
    let mut data = HashMap::new();

    for sub_str in s.split('&') {
      let mut key = sub_str;
      let mut val = "";

      if let Some(i) = sub_str.find('=') {
        val = &sub_str[i + 1..];
        key = &sub_str[..i];
      }

      data
        .entry(key)
        .and_modify(|curr: &mut Value| match curr {
          Value::Multi(vec) => vec.push(val),
          Value::Single(prev) => {
            *curr = Value::Multi(vec![prev, val]);
          }
        })
        .or_insert(Value::Single(val));
    }

    QueryString { data }
  }
}
