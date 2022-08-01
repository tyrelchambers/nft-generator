// const getRarityWeight = (_str) => {
//   let nameWithoutExtension = _str.slice(0, -4);
//   var rarity = Number(nameWithoutExtension.split(rarityDelimiter).pop());
//   if (isNaN(rarity)) {
//     rarity = 1;
//   }

//   return rarity;
// };

use std::{fmt, str::Chars};

pub fn remove_extension(str: &str) -> String {
    let mut name_without_extension: String = str.to_string();
    name_without_extension.pop();
    name_without_extension.pop();
    name_without_extension.pop();
    name_without_extension.pop();
    return name_without_extension;
}

pub fn get_rarity_weight(str: &str) -> i32 {
    let clean_name: String = remove_extension(&str);
    let rarity: Vec<_> = clean_name.split(crate::RARITY_DELIMITER).collect();

    let rarity_weight: i32 = rarity[1].parse::<i32>().unwrap_or(1);

    return rarity_weight;
}
