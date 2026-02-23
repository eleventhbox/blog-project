mod handlers;
mod server;

pub mod blog {
    tonic::include_proto!("blog");
}

fn main() {}
