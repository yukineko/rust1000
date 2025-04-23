use serde_json::json;
use serde_json::Value;
fn main() -> Result<(), serde_json::Error> {
    let json_str = r#"{"name": "John Doe", "age": 30, "is_student": false}"#;
    let parsed:Value = serde_json::from_str(json_str)?;
    println!("{}", parsed["name"].to_string());
    let j = json!(
        {
            "name": "John Doe",
            "age": 30,
            "is_student": false,
            "courses": ["Math", "Science"]
        }
    );

    println!("{}", j["name"]);
    Ok(())
}