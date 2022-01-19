use bollard::Docker;
use bollard::image::ListImagesOptions;
use std::default::Default;

#[tokio::main]
async fn main() {
    let docker = Docker::connect_with_socket_defaults().unwrap();
    let images = &docker.list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        })).await.unwrap();

        for image in images {
            println!("-> {:?}", image);
        }
}
