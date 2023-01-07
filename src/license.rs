use serde::Deserialize;


// all structs are from https://api.github.com/licenses
#[derive(Debug, Deserialize)]
pub struct License {
    pub key: String,
    pub name: String,
    pub spdx_id: String,
    pub url: String,
    pub node_id: String,
}

#[derive(Debug, Deserialize)]
pub struct LicenseContent {
    pub key: String,
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
    pub conditions: Vec<String>,
    pub limitations: Vec<String>,
    pub body: String,
}



impl LicenseContent  {
pub fn fetch_license_content(url:&String) -> LicenseContent {
    let license:LicenseContent = match ureq::get(&url).call() {
        Ok(res) => res.into_json().unwarp(),
        Err(error) => panic!("Unable to fetch lice contet :{}",error)
    };
    license
}
}



#[drive(Debug,Deserialize)] 
pub struct Licenses {
    pub licenses:Vec<License>,
}
impl License { 

pub fn fetch_licenses()  -> Licenses {

    let body :Vec<license>= match ureq::get("")
}


}
