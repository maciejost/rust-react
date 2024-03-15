mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn validate_name(name: &str) -> String {
    if name.len() > 3 {
        return format!("Gyldig navn");
    } else {
        return format!("Ugyldig navn");
    }
}

#[wasm_bindgen]
pub fn validate_email(email: &str) -> String {
    if email.contains("@fremtind.no") {
        return format!("Gyldig epostadresse");
    } else {
        return format!("Ugyldig epostadresse");
    }
}


#[wasm_bindgen]
pub fn validate_age(age: i32) -> String {
    if age > 18 {
        return format!("Du er gammel nok");
    } else {
        return format!("Du er for ung");
    }
}
