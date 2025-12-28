use std::fs::read_to_string;

fn main() {
    let sokyoei_data_dir = rose::get_sokyoei_data_dir();
    let yaml_file_path = sokyoei_data_dir.join("Ahri").join("Ahri.yaml");
    let data = read_to_string(yaml_file_path);
}
