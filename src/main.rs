use rocket::http::uri::Absolute;
use rocket::response::status::{NoContent, self, Created, Conflict};
use rocket::tokio::fs::{File, self};
use rocket::{Data, data::ToByteUnit};
mod paste_id;
mod authentication;

use paste_id::PasteId;

use crate::authentication::ApiKey;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
      
      PUT /<filename>
          
          same as POST / but makes a file with the appended name
          requires authentication
    "
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> std::io::Result<File> {
    File::open(id.file_path()).await
}

// In a real application, these would be retrieved dynamically from a config.
const ID_LENGTH: usize = 5;
const HOST: Absolute<'static> = uri!("http://0x0.anima.nz");

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    paste.open(32.mebibytes()).into_file(id.file_path()).await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}

#[put("/<path>", data = "<paste>")]
async fn upload_path(paste: Data<'_>, path: &str, auth: ApiKey<'_>) -> Result<String, status::Conflict<String>> {
    let id = PasteId::from(path);
    if fs::try_exists(id.file_path()).await.unwrap() {
        Err(Conflict(Some("Cannot PUT: id exists".to_string())))
    } else {
        paste.open(512.mebibytes()).into_file(id.file_path()).await.unwrap();
        Ok(uri!(HOST, retrieve(id)).to_string())
    }
}

#[delete("/<path>")]
async fn delete(path: &str, auth: ApiKey<'_>) -> std::io::Result<String> {
    let id = PasteId::from(path);
    if id.file_path().exists() {
        fs::remove_file(id.file_path()).await?
    }
    Ok(format!("Deleted: {}", id.file_path().file_name().unwrap().to_str().unwrap()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, retrieve, upload, upload_path, delete])
}