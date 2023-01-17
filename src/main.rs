#[derive(Debug)]
pub struct MyPublicStruct {
    name: String,
}

impl<'de> serde::Deserialize<'de> for MyPublicStruct {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        
        #[derive(serde::Deserialize)]
        struct BackwardsCompatibleStableStruct {
            first_name: String,
            last_name: String,
        }

        let stable_struct = BackwardsCompatibleStableStruct::deserialize(deserializer)?;
        Ok(Self {
            name: format!("{} {}", stable_struct.first_name, stable_struct.last_name),
        })
    }
}

fn main() {
    let q: MyPublicStruct = serde_json::from_str(r#"{"first_name": "A", "last_name": "B"}"#).unwrap();
    println!("{:#?}", q);
    // Output:
    // MyPublicStruct {
    //     name: "A B",
    // }
}
