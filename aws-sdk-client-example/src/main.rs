/// Exemplo de Client AWS usando o crate Rusoto.
/// 
/// ## Ideias
/// - Transformar este exemplo em uma CLI usando o `clap` e o `colored`
/// - Usar interface semelhante ao awscli da Amazon (mas apenas para o S3)
/// - Usar .env no diretório corrente, ao invés do `~/.aws/credentials`
/// - Opções de deploy: deb/snap/docker
/// 
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_s3;

use rusoto_s3::{S3, S3Client};

fn main() {
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
