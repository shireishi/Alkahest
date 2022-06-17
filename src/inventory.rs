#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::{io, fmt, option, vec};

// https://doc.rust-lang.org/rust-by-example/trait.html
// https://doc.rust-lang.org/rust-by-example/generics/bounds.html

pub static mut ITEM_VECTOR: Vec<Item> = Vec::new();

pub fn new() -> Vec<Item> {
    Vec::new()
}

fn gen_hash(data: String) -> String {
    let char_vec: Vec<char> = data.chars().collect(); // convert the data string into a character vec
    let vec_size: usize = char_vec.len(); // store the vector size
    let mut hex_vec: Vec<String> = Vec::with_capacity(vec_size); // allocate the hex vector
    
    for i in 0 .. vec_size { // iterate through each character
        let to_app = char_vec[i] as u8 / vec_size as u8; // set to_app to the ordinal value of the char divided by the length of the data set
        hex_vec.push(std::format!("{:x}", to_app)); // append the string formatted hex value to the hex vector
    };

    String::from(hex_vec.join("")) // combine the hex vector
}

fn generate_attributes(attrs: Vec<ItemAttributes>) -> u8 {
    let mut return_value: u8 = 0;

    for attr in attrs {
        let shift_val: u8 = 1 << attr as i32;
        return_value |= shift_val;
        drop(shift_val);
    };
    return_value
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemAttributes {
    Sword,
    Axe,
    Shield,
    Misc
}
impl fmt::Display for ItemAttributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, PartialEq, Default, Eq)]
pub struct Item {
    pub item_name: String,
    pub item_id: String,
    pub attributes: Vec<ItemAttributes>
}
impl Item {
    pub fn new(name: String, attrs: Vec<ItemAttributes>) -> Item {
        let id: String = gen_hash(name.clone());
        let to_ret: Item = Item {
            item_name: name,
            item_id: id,
            attributes: attrs
        };
        unsafe {
            if !ITEM_VECTOR.contains(&to_ret.clone()) {
                ITEM_VECTOR.push(to_ret.clone());
            }
        };
        to_ret
    }
    pub fn copy(&self) -> Item {
        self.clone()
    }
}
impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let attributes: u8 = generate_attributes(self.attributes.clone());
        let binary_value: String = std::format!("{:#010b}", attributes);
        let mut attribute_vec: Vec<String> = Vec::new();

        for attr in self.attributes.clone() {
            attribute_vec.push(attr.to_string());
        };

        let attribute_string: String = attribute_vec.join(", ");
        let attr_field: String = std::format!("{} {} {}", binary_value, attributes, attribute_string);

        f.debug_struct("Item")
            .field("Name", &self.item_name)
            .field("Item ID", &self.item_id)
            .field("Attributes", &attr_field)
            .finish()
    }
}
// todo: write a display implementation for Item