extern crate serde_json;
extern crate serenity;

use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

struct CopyPastaCollection {
  pasta_collection: HashMap<String, String>,
}

impl CopyPastaCollection {
  pub fn new() -> Self {
    let f = match File::open("pastas.json") {
      Ok(file) => file,
      Err(_) => {
        return CopyPastaCollection {
          pasta_collection: HashMap::new(),
        }
      }
    };
    let mut reader = BufReader::new(f);
    let mut raw_json = String::new();
    let _ = match reader.read_to_string(&mut raw_json) {
      Ok(_) => 0,
      Err(_) => {
        return CopyPastaCollection {
          pasta_collection: HashMap::new(),
        }
      }
    };

    return match serde_json::from_str(raw_json.as_str()) {
      Ok(data) => CopyPastaCollection {
        pasta_collection: data,
      },
      Err(_) => CopyPastaCollection {
        pasta_collection: HashMap::new(),
      },
    };
  }
}

struct Handler;

impl EventHandler for Handler {
  fn message(&self, _: Context, msg: Message) {
    if msg.content == "!ping" {
      if let Err(why) = msg.channel_id.say("Pong!") {
        println!("Error sending message: {:?}", why);
      }
    }
  }

  fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}

fn main() {
  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
  let mut client = Client::new(&token, Handler).expect("Err creating client");
  if let Err(why) = client.start() {
    println!("Client error: {:?}", why);
  }
}
