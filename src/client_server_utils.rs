use get_if_addrs::get_if_addrs;
use openssl::pkey::Private;
use openssl::rsa::Rsa;
use std::fs;
use std::str;

/// Get the first non-loopback IPv4 address of the machine
pub fn get_ipv4() -> Option<String> {
    let if_addrs = get_if_addrs().ok()?;
    for iface in if_addrs {
        if !iface.is_loopback() {
            if let std::net::IpAddr::V4(ip) = iface.ip() {
                return Some(ip.to_string());
            }
        }
    }
    None
}

/// Decrypts the password from a file using the private key
pub fn decrypt_password_rsa(
    password: &str,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {


    print!("PASSWORD AS IT IS: {}\n\n", password);
    // Get project root directory
    let project_root = std::env::current_dir().expect("Failed to get current directory");

    // Store the joined paths in variables to extend their lifetimes
    let private_key_path_buf = project_root.join("src/keys/private_key.pem");
    let encrypted_password_path_buf = project_root.join("src/keys/password.enc");

    let private_key_path = private_key_path_buf.to_str().unwrap();
    let encrypted_password_path = encrypted_password_path_buf.to_str().unwrap();

    // Read the private key from the file
    let private_key_pem = fs::read_to_string(private_key_path)?;
    let private_key = Rsa::<Private>::private_key_from_pem(private_key_pem.as_bytes())?;

    // Read the encrypted data from the file
    let encrypted_data = fs::read(encrypted_password_path)?;

    // Decrypt the data
    let mut buffer = vec![0; private_key.size() as usize];
    let decrypted_size =
        private_key.private_decrypt(&encrypted_data, &mut buffer, openssl::rsa::Padding::PKCS1)?;

    // Convert the decrypted data to a string
    let decrypted_password = String::from_utf8_lossy(&buffer[..decrypted_size]).to_string();

    // Trim both passwords to remove any extra whitespace or newlines
    let decrypted_password_trimmed = decrypted_password.trim();
    let input_password_trimmed = password.trim();
    

    // Compare the decrypted password with the input password
    print!("Decrypted password: {}\n\n", decrypted_password_trimmed);
    println!("Input password: {}\n\n\n", input_password_trimmed);

    // Check if the decrypted password matches the input password
    print!("TRUE OR FALSE: {}\n", input_password_trimmed == decrypted_password_trimmed);
    Ok(input_password_trimmed == decrypted_password_trimmed)
}
