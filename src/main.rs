use std::error::Error;

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct Data {
    records: Vec<Record>,
}

#[derive(Serialize, Deserialize)]
struct Record {
    event_id: String,
}

#[derive(Serialize, Deserialize)]
struct EventLog {
    id: String,
    delivered: bool,
}

/// Simple program to redeliver undelivered Hasura events
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The hasura URL
    url: String,

    /// Hasura's admin secret. If not present, use HASURA_GRAPHQL_ADMIN_SECRET env var
    #[arg(short, long)]
    admin_secret: Option<String>,

    /// The file in which the failed events are stored. If it is not present, will fetch the data
    #[arg(short, long)]
    data_file: Option<String>,

    /// The event trigger name
    #[arg(short, long)]
    event_trigger_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();
    let url = &cli.url;
    let admin_secret = match cli.admin_secret {
        Some(a) => a.clone(),
        None => std::env::var("HASURA_GRAPHQL_ADMIN_SECRET")?,
    };

    let get_event_logs_payload = json!({
        "type": "pg_get_event_logs",
        "args": {
            "name": &cli.event_trigger_name,
            "source": "default",
            "status": "processed",
            "limit": 100,
            "offset": 0
        }
    });

    let data: Vec<EventLog> = match cli.data_file {
        Some(file) => {
            let file = std::fs::read(file)?;
            serde_json::from_slice(&file)?
        }
        None => {
            let event_logs = client
                .post(url)
                .json(&get_event_logs_payload)
                .send()
                .await?
                .text()
                .await?;

            serde_json::from_str(&event_logs)?
        }
    };

    let records: Vec<Record> = data
        .iter()
        .filter_map(|evt| {
            if evt.delivered {
                return None;
            }
            Some(Record {
                event_id: evt.id.clone(),
            })
        })
        .collect();

    for rec in records {
        let res = client
            .post(url)
            .json(&json!({
                "type" : "pg_redeliver_event",
                "args" : {
                    "event_id": rec.event_id
                }
            }))
            .header("x-hasura-admin-secret", &admin_secret)
            .header("x-hasura-role", "admin")
            .send()
            .await?;

        println!("Got response:\n{res:?}");
    }

    Ok(())
}
