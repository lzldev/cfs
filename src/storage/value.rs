use rkyv::{Archive, Deserialize, Serialize};
use std::fmt::Display;

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq, Clone)]
#[rkyv(
    // This will generate a PartialEq impl between our unarchived
    // and archived types
    compare(PartialEq),
    // Derives can be passed through to the generated type:
    derive(Debug),
)]
pub enum StoreValue {
	Value(String),
	List(Vec<String>),
}

impl Display for StoreValue {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			StoreValue::Value(v) => write!(f, "{}", v),
			StoreValue::List(items) => {
				let mut array_string = String::new();

				for (idx, i) in items.iter().enumerate() {
					array_string.push_str(&format!("{}", i));
					if idx < (items.len() - 1) {
						array_string.push(',');
					}
				}

				write!(f, "[{}]", array_string)
			}
		}
	}
}
