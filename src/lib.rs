use std::fs;
use c2pa::{create_signer, SigningAlg, Manifest};
use serde_json::json;
use std::ffi::{CStr};
use std::os::raw::c_char;
//use tempfile::NamedTempFile;
use serde::Deserialize;


#[derive(Debug, Default, Deserialize)]
struct ManifestDef {
    #[serde(flatten)]
    manifest: Manifest
}

#[no_mangle]
pub extern "C" fn c2pa_sign(
    c_manifest_file: *const c_char,
    c_input_file: *const c_char,
    c_output_file: *const c_char,
    c_cert_file: *const c_char,
    c_key_file: *const c_char) {

    let manifest_file = unsafe { CStr::from_ptr(c_manifest_file).to_str().unwrap_or("") };
    let input_file = unsafe { CStr::from_ptr(c_input_file).to_str().unwrap_or("") };
    let output_file = unsafe { CStr::from_ptr(c_output_file).to_str().unwrap_or("") };
    let cert_file = unsafe { CStr::from_ptr(c_cert_file).to_str().unwrap_or("") };
    let key_file = unsafe { CStr::from_ptr(c_key_file).to_str().unwrap_or("") };
    


    println!("Manifest file: {}", manifest_file);
    println!("Input file: {}", input_file);
    println!("Output file: {}", output_file);
    println!("Certificate file: {}", cert_file);
    println!("Key file: {}", key_file);


    let manifest_json = json!({
        "claim_generator_info": [
            {
                "name": "c2pa_test",
                "version": "1.0.0"
            }
        ],
        "title": "Test_Manifest"
    }).to_string();


    // Create a ps256 signer using certs and key files
    let signer = match create_signer::from_files(cert_file, key_file, SigningAlg::Ps256, None){
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to create signer: {}", e);
            return; // Exit early if there's an error
        }
    };

    // Create a temporary file
/*    let temp_file = match NamedTempFile::new() {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create temporary file: {}", e);
            return;
        }
    };
    let temp_path = temp_file.path().to_owned();
    let  temp_file_name: &str = &temp_path.to_str().unwrap();
*/
    let temp_file_name="/tmp/test.mp4";

    // read the manifest information
    let manifest_def: ManifestDef = serde_json::from_slice(manifest_json.as_bytes()).expect("error");
    let mut manifest = manifest_def.manifest;

    println!("temp file file: {}", temp_file_name);

    manifest
    .embed(&input_file, &temp_file_name, signer.as_ref()).expect("error");

        
    if let Err(e) = fs::rename(&temp_file_name, output_file) {
        eprintln!("Failed to replace the original file: {}", e);
    }
}