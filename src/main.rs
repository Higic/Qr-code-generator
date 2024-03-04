use std::env;

use qrcode::QrCode;
use image::Luma;
use sha2::{Sha256, Digest};

fn generate_qr_code_to_terminal(data: &str) -> &str {
    let code = QrCode::new(&data).unwrap();
    let string = code.render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{}", string);
    data

}

fn generate_qr_code_to_png(data: &str) -> String {
    let code = QrCode::new(&data).unwrap();
    
    // Name generation and hashing
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();
    let name = format!("{:x}", result) + ".png";

    let image = code.render::<Luma<u8>>().build();
    image.save(&name).unwrap();
    println!("Saved to {}", &name);
    name
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 { generate_qr_code_to_terminal(&args[1]);} 
    if args.len() == 3 && args[2] == "--png" {generate_qr_code_to_png(&args[1]);}
}


#[cfg(test)]
mod tests {
    
    #[test]
    fn test_generate_qr_code_to_terminal() {
        let data = "hello_world";
        assert_eq!(generate_qr_code_to_terminal(data), (data));
    }
    
    #[test]
    fn test_generate_qr_code_to_png() {
        let data = "hello_world";
        // hello_world hashes to 35072c1ae546350e0bfa7ab11d49dc6f129e72ccd57ec7eb671225bbd197c8f1
        assert_eq!(generate_qr_code_to_png(data), "35072c1ae546350e0bfa7ab11d49dc6f129e72ccd57ec7eb671225bbd197c8f1.png");
    }
    
    #[test]
    fn test_fail_qr_code_to_png() {
        let data = "hello_world";
        assert_ne!(generate_qr_code_to_png(&data), data);
    }
    
    #[test]
    fn test_main_no_png() {
        let args = vec!["cargo".to_string(), "run".to_string(), "hello_world".to_string()];
        assert_eq!(main(), ());
    }
    
    #[test]
    fn test_main_png() {
        let args = vec!["cargo".to_string(), "run".to_string(), "hello_world".to_string(), "--png".to_string()];
        assert_eq!(main(), ());
    }
    use super::*;
}