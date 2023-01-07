
mod helpers;
mod license;

fn main() {
    let licenses=license::Licenses::fetch_licenses();
 let license = helpers::select(&licenses.get_license_names());
  let license_content = &licenses.get_license_from_name(&license);
    helpers::fill_content(license_content);


}
