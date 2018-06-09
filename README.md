# FileServe
> (WIP) Personal-use only File sharing/url shortener Service written in Rust.

## Configure 
modify `.env` file.

## How to run
You need to install rustup and rust-nightly.
```bash
$ rustup override set nightly
$ cargo run --release.
```
it will host on `http://localhost:8000`

## ShareX Compatibles
This program is made by personal ShareX upload service.

Change `__hostname__` and `__token_key__` to your own setting from `.env`.

Optional : change `__name__` to your destination name.
### File Upload
```
{
  "Name": "__name__",
  "DestinationType": "ImageUploader, TextUploader, FileUploader",
  "RequestURL": "__hostname__/v1/api",
  "FileFormName": "filedata",
  "Headers": {
    "x-token-key": "__token_key__",
    "x-request-type": "fileupload",
    "x-file-name": "$filename$"
  },
  "URL": "$json:link$"
}
```
### Url Shortener
```
{
  "Name": "__name__",
  "DestinationType": "URLShortener",
  "RequestURL": "__hostname__/v1/api",
  "Headers": {
    "x-token-key": "__token_key__",
    "x-request-type": "urlshortener",
    "x-url-target": "$input$"
  },
  "URL": "$json:link$"
}
```