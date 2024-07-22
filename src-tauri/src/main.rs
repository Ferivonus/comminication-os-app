// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod server;
use reqwest::Client;
use std::ptr::null;
use std::thread;

use futures::stream::StreamExt;
use libp2p::{gossipsub, mdns, noise, swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux};
use once_cell::sync::Lazy;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::Manager;
use tokio::{io, io::AsyncBufReadExt, select};
use tracing_subscriber::EnvFilter;

// Global static for the Swarm
static SWARM: Lazy<Mutex<Option<libp2p::Swarm<MyBehaviour>>>> = Lazy::new(|| Mutex::new(None));

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn fetch_wailing_example_data() -> Result<String, String> {
    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:4875/wailing/example")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let body = response.text().await.map_err(|e| e.to_string())?;
        Ok(body)
    } else {
        Err(format!("HTTP error: {}", response.status()))
    }
}

#[derive(NetworkBehaviour)]
struct MyBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

async fn run_node(app_handle: tauri::AppHandle) -> Result<(), Box<dyn Error>> {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init();

    let (mut swarm, topic) = create_swarm().await?;

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    // Start listening on the network addresses
    swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Store the swarm instance in the global state
    *SWARM.lock().unwrap() = Some(swarm);
    let mut lock = SWARM.lock().unwrap();

    loop {
        select! {
            Ok(Some(line)) = stdin.next_line() => {
                let mut lock = SWARM.lock().unwrap();
                if let Some(swarm) = lock.as_mut() {
                    if let Err(e) = publish_message(swarm, &topic, line).await {
                        app_handle.emit_all("node-error", format!("Publish error: {e:?}")).unwrap();
                    }
                }
            }
            event = {
          let mut lock = SWARM.lock().unwrap();

          if let Some(swarm) = lock.as_mut() {
                    swarm.select_next_some()
                } else {
                    // Handle the case where swarm is None (e.g., log an error, retry)
                    eprintln!("Error: Swarm instance is missing");
                    return Err("Swarm instance is missing".into());
                }
            } => {
                handle_event(event, &app_handle).await;
            }
        }
    }
}

async fn publish_message(
    swarm: &mut libp2p::Swarm<MyBehaviour>,
    topic: &gossipsub::IdentTopic,
    line: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    swarm
        .behaviour_mut()
        .gossipsub
        .publish(topic.clone(), line.as_bytes())?;
    Ok(())
}

async fn handle_event(event: SwarmEvent<MyBehaviourEvent>, app_handle: &tauri::AppHandle) {
    match event {
        SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
            for (peer_id, _multiaddr) in list {
                app_handle
                    .emit_all(
                        "node-event",
                        format!("mDNS discovered a new peer: {peer_id}"),
                    )
                    .unwrap();
                // Add explicit peer
                SWARM
                    .lock()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .behaviour_mut()
                    .gossipsub
                    .add_explicit_peer(&peer_id);
            }
        }
        SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
            for (peer_id, _multiaddr) in list {
                app_handle
                    .emit_all(
                        "node-event",
                        format!("mDNS discover peer has expired: {peer_id}"),
                    )
                    .unwrap();
                // Remove explicit peer
                SWARM
                    .lock()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .behaviour_mut()
                    .gossipsub
                    .remove_explicit_peer(&peer_id);
            }
        }
        SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
            propagation_source: peer_id,
            message_id: id,
            message,
        })) => app_handle
            .emit_all(
                "node-event",
                format!(
                    "Got message: '{}' with id: {id} from peer: {peer_id}",
                    String::from_utf8_lossy(&message.data),
                ),
            )
            .unwrap(),
        SwarmEvent::NewListenAddr { address, .. } => {
            app_handle
                .emit_all(
                    "node-event",
                    format!("Local node is listening on {address}"),
                )
                .unwrap();
        }
        _ => {}
    }
}

#[tauri::command]
async fn start_message_node(app_handle: tauri::AppHandle) -> Result<(), String> {
    let result = run_node(app_handle).await;
    result.map_err(|e| e.to_string())
}

// Global broadcaster for message passing
static BROADCAST: Lazy<Arc<tokio::sync::broadcast::Sender<String>>> = Lazy::new(|| {
    let (tx, _) = tokio::sync::broadcast::channel(100); // Adjust buffer size as needed
    Arc::new(tx)
});

#[tauri::command]
async fn send_message(message: String) -> Result<(), String> {
    let tx = BROADCAST.clone();
    tx.send(message).map_err(|e| e.to_string())?;
    Ok(())
}

async fn create_swarm(
) -> Result<(libp2p::Swarm<MyBehaviour>, gossipsub::IdentTopic), Box<dyn Error>> {
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_quic()
        .with_behaviour(|key| {
            let message_id_fn = |message: &gossipsub::Message| {
                let mut s = DefaultHasher::new();
                message.data.hash(&mut s);
                gossipsub::MessageId::from(s.finish().to_string())
            };

            let gossipsub_config = gossipsub::ConfigBuilder::default()
                .heartbeat_interval(Duration::from_secs(10))
                .validation_mode(gossipsub::ValidationMode::Strict)
                .message_id_fn(message_id_fn)
                .build()
                .map_err(|msg| io::Error::new(io::ErrorKind::Other, msg))?;

            let gossipsub = gossipsub::Behaviour::new(
                gossipsub::MessageAuthenticity::Signed(key.clone()),
                gossipsub_config,
            )?;

            let mdns =
                mdns::tokio::Behaviour::new(mdns::Config::default(), key.public().to_peer_id())?;
            Ok(MyBehaviour { gossipsub, mdns })
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    let topic = gossipsub::IdentTopic::new("test-net");
    swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

    Ok((swarm, topic))
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            // Initialize the broadcast channel and spawn task
            let tx = BROADCAST.clone();
            tokio::spawn(async move {
                let mut rx = BROADCAST.subscribe();
                while let Ok(message) = rx.recv().await {
                    println!("Received message from Svelte: {}", message);
                }
            });

            // Initialize server in a separate thread
            thread::spawn(move || {
                server::init(app_handle.clone()).unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            fetch_wailing_example_data,
            greet,
            start_message_node,
            send_message,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
