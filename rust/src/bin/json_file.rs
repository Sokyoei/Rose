use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sokyoei_data_dir = rose::get_sokyoei_data_dir();
    let json_file_path = sokyoei_data_dir.join("Ahri").join("Ahri.json");
    let data = read_to_string(json_file_path)?;

    let json: serde_json::Value = serde_json::from_str(&data)?;

    println!("{}", serde_json::to_string_pretty(&json)?);
    Ok(())
}
