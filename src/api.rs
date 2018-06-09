use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::Arc;
use std::env;

use rocket::Outcome;
use rocket::Data;
use rocket::http::{Status, ContentType};
use rocket::response::{content, Failure};
use rocket::request::{self, Request, FromRequest};

use multipart::server::Multipart;
use multipart::server::save::SaveResult::*;

use id;

pub struct ApiRequestHeader {
    pub token: String,
    pub reqtype: String,
    pub filename: String,
    pub urldata: String,
}

lazy_static! {
    static ref HOSTNAME: String = env::var("HOSTNAME").unwrap_or("http://localhost:8000".to_string());
    static ref TOKEN_KEY: String = env::var("TOKEN_KEY").unwrap_or("valid_api_key".to_string());
}

/// Returns true if `key` is a valid API key string.
fn is_valid(key: &str) -> bool {
    key == &*TOKEN_KEY
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiRequestHeader {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiRequestHeader, ()> {
        let token: Vec<_> = request.headers().get("x-token-key").collect();
        let reqtype: Vec<_> = request.headers().get("x-request-type").collect();
        let mut filename: Vec<_> = request.headers().get("x-file-name").collect();
        let mut urldata: Vec<_> = request.headers().get("x-url-target").collect();

        if token.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        if reqtype.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        if filename.len() != 1 {
            filename = vec![""];
        }
        if urldata.len() != 1 {
            urldata = vec![""];
        }

        let key = token[0];
        if !is_valid(token[0]) {
            return Outcome::Forward(());
        }

        return Outcome::Success(ApiRequestHeader {
            token: key.to_string(),
            reqtype: reqtype[0].to_string(),
            filename: filename[0].to_string(),
            urldata: urldata[0].to_string()
        });
    }
}


#[post("/", data = "<data>")]
pub(crate) fn api_index(ct: &ContentType, data: Data, req: ApiRequestHeader) -> Result<content::Json<String>, Failure> {
    if req.reqtype == "fileupload" {
        file_upload(ct, data, req)
    } else if req.reqtype == "urlshortener" {
        url_shortener(data, req)
    } else { return Err(Failure(Status::BadRequest)) }
}

fn url_shortener(_data: Data, req: ApiRequestHeader) -> Result<content::Json<String>, Failure> {
    let id = id::ItemID::new(8, true);
    let filename = format!("content/urls/{id}", id = id);
    let url = format!("{host}/r/{id}", host = &*HOSTNAME, id = id);

    // Write the paste out to the file and return the URL.
    let mut file = File::create(Path::new(&filename)).ok().unwrap();
    file.write_all(req.urldata.as_bytes()).ok();
    Ok(content::Json(format!("{{ 'success': true, 'link': '{url}' }}", url = url)))
}

fn file_upload(content_type: &ContentType,data: Data, req: ApiRequestHeader) -> Result<content::Json<String>, Failure> {
    if !content_type.is_form_data() {
        return Err(Failure(Status::BadRequest))
    }

    let (_, boundary) = content_type.params().find(|&(k, _)| k == "boundary").ok_or_else(
        || Failure(Status::BadRequest)
    )?;
   

    let id = format!("{}.{}", id::ItemID::new(8, false), req.filename.split(".").last().unwrap_or("bin"));
    let filename = format!("content/files/{id}", id = id);
    let url = format!("{host}/i/{id}", host = &*HOSTNAME, id = id);

    let mut file = File::create(Path::new(&filename)).ok().unwrap();
    let mut content: Vec<u8> = Vec::new();

    match Multipart::with_body(data.open(), boundary).save().temp() {
        Full(entries) => {
            let r = entries.fields.get(&Arc::new(String::from("filedata"))) // HashMap
                    .ok_or_else(
                        || Failure(Status::InternalServerError)
                    )?[0].data.readable().ok().ok_or_else( // Saved Data
                        || Failure(Status::InternalServerError)
                    )?.read_to_end(&mut content).ok().ok_or_else( // Write to folder
                        || Failure(Status::InternalServerError)
                    )?;

            Ok(r)
        },
        _ => Err(Failure(Status::UnprocessableEntity))
    }?;
    // _file.read_to_end(&mut content).ok().ok_or_else(
    //     || Failure(Status::InternalServerError)
    // )?;

    file.write_all(&content).ok().ok_or_else(
        || Failure(Status::InternalServerError)
    )?;

    return Ok(content::Json(format!("{{ 'success': true, 'link': '{url}' }}", url = url)))
}