use clap::Parser;
use redis::Commands;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

#[derive(clap::Parser)]
struct Args {
   #[clap(subcommand)]
   command: SubCommands,

    /// Address of Redis server
    #[clap(short, long, default_value = "redis://127.0.0.1/")]
    redis_address: String,
}

#[derive(clap::Subcommand)]
enum SubCommands  {
   /// Clones repos
   #[clap(arg_required_else_help = true)]
   Clone {
       /// The remote to clone
       #[clap(value_parser)]
       out_path: String,

       /// The redis key of stored file
       #[clap(value_parser)]
       file_key: String,
   },
   /// Pushes things
   #[clap(arg_required_else_help = true)]
   Push {
       /// The remote to target
       #[clap(value_parser)]
       file_path: String,

       /// The redis key to store the file as
       #[clap(value_parser)]
       file_key: String,
   },
   /// Dletes things
   #[clap(arg_required_else_help = true)]
   Delete {
       /// The redis key to delete from the DB
       #[clap(value_parser)]
       file_key: String,
   },
}

fn main() {
    let args = Args::parse();

    let addr = args.redis_address;

    match args.command {
        SubCommands::Clone { out_path, file_key } => {
            download_file(addr, out_path, file_key);
        }
        SubCommands::Push { file_path, file_key } => {
            upload_file(addr, file_path, file_key);
        }
        SubCommands::Delete { file_key } => {
            delete_file(addr, file_key);
        }
    }
}

/// Downloads a file from Redis
fn download_file(redis_address: String, out_path: String, file_key: String) {
    println!("Downloading RedisFS:{} to {}", file_key, out_path);

    // Connect to Redis
    let client = redis::Client::open(redis_address)
        .expect("Failed to create Redis client");
    let mut con = client.get_connection()
        .expect("Failed to connect to Redis");

    // Store the file in the DB
    let contents: Vec<u8> = con.get(format!("RedisFS:{}", &file_key))
        .expect("Failed to get key value");

    // Create the file
    let mut out_file = File::create(out_path)
        .expect("Failed to create file");
    
    // Write the key contents to the file
    out_file.write_all(&contents)
        .expect("Failed to write to file");
    

    println!("Upload complete. Saved as RedisFS:{}", &file_key);
}

/// Uploads a file to the Redis DB
fn upload_file(redis_address: String, file_path: String, file_key: String) {
    println!("Pushing {} to DB as {}", file_path, file_key);

    // Connect to Redis
    let client = redis::Client::open(redis_address)
        .expect("Failed to create Redis client");
    let mut con = client.get_connection()
        .expect("Failed to connect to Redis");

    // Read the file's contents
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    // Store the file in the DB
    let _ : () = con.set(format!("RedisFS:{}", &file_key), contents)
        .expect("Failed to set key");

    println!("Upload complete. Saved as RedisFS:{}", &file_key);
}

/// Removes a file from the Redis DB
fn delete_file(redis_address: String, file_key: String) {
    println!("Removing {} from DB", file_key);

    // Connect to Redis
    let client = redis::Client::open(redis_address)
        .expect("Failed to create Redis client");
    let mut con = client.get_connection()
        .expect("Failed to connect to Redis");

    // Store the file in the DB
    let _ : () = con.del(format!("RedisFS:{}", &file_key))
        .expect("Failed to delete key");

        println!("Removed {} from DB", file_key);
}
