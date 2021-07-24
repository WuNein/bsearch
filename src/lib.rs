#[macro_use]
extern crate lazy_static;
extern crate json;
extern crate wasm_bindgen;


use wasm_bindgen::prelude::*;
use jieba_rs::Jieba;

lazy_static! {
    static ref JIEBA : jieba_rs::Jieba = Jieba::new();
}

#[wasm_bindgen]
pub fn cut(sent: String) -> String{
    let words = JIEBA.cut(&sent, false);
    
    // let result = words.join("\\");
    let result = json::stringify(words);

    return result;
}

#[wasm_bindgen]
pub fn cut_for_search(sent: String) -> String{
    let words = JIEBA.cut_for_search(&sent, false);
    
    // let result = words.join("\\");
    let result = json::stringify(words);

    return result;
}

#[wasm_bindgen]
pub fn cut_for_tag(sent: String) -> String{
    let words = JIEBA.tag(&sent, false);
    // let mut result :String = "".to_string();
    let mut result = json::JsonValue::new_array();
    for elem in words {
        let temp = json::object!{
            "word" => elem.word,
            "tag" => elem.tag
        };
        // let tempJson = temp.dump().to_string();
        result.push(temp);
    }
    let result2 = json::stringify(result);
    return result2;
}

fn main(){
}