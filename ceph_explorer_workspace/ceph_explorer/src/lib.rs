extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_s3;

use rusoto_s3::{S3, S3Client};

pub fn print_buckets() {
    let request_dispatcher = rusoto_core::HttpClient::new()
        .expect("Erro ao criar request dispatcher");
    let credentials_provider = rusoto_credential::EnvironmentProvider::default();
    let region = rusoto_core::Region::Custom {
        name: String::from("us-west-2"),
        endpoint: String::from("")
    };

    let s3: S3Client = rusoto_s3::S3Client::new_with(
        request_dispatcher,
        credentials_provider,
        region
    );

    match s3.list_buckets().sync() {
        Ok(buckets) => println!("Buckets found: {:?}", buckets),
        Err(e) => println!("Error while listing buckets: {}", e)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
