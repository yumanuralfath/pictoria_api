#[macro_use]
extern crate rocket;

use yumana_api::rocket;

#[launch]
fn launch() -> _ {
    rocket()
}
