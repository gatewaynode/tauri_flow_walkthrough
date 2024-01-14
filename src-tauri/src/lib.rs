use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub node_type: String,
    pub position: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    pub edge_type: String,
    pub source: String,
    pub target: String,
    pub animated: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_edges, get_nodes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Use Rusqlite to get all values from the nodes table in our SQLite DB located at /data/flow_demo.sqlite3
#[tauri::command]
fn get_nodes() -> Vec<Node> {
    println!("Rust call get_nodes:");
    let conn = Connection::open("../data/flow_demo.sqlite3").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM nodes").unwrap();
    let node_iter = stmt
        .query_map([], |row| {
            Ok(Node {
                id: row.get(0)?,
                node_type: row.get(1)?,
                position: row.get(2)?,
                data: row.get(3)?,
            })
        })
        .unwrap();
    let mut nodes: Vec<Node> = node_iter.map(|node| node.unwrap()).collect();
    println!("DB nodes: {:?}", nodes);
    nodes
}

//  Similar to get_nodes(), query the edges table in our SQLite DB located at /data/flow_demo.sqlite3
#[tauri::command]
fn get_edges() -> Vec<Edge> {
    println!("Rust call get_edges");
    let conn = Connection::open("../data/flow_demo.sqlite3").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM edges").unwrap();
    let edge_iter = stmt
        .query_map([], |row| {
            Ok(Edge {
                id: row.get(0)?,
                edge_type: row.get(1)?,
                source: row.get(2)?,
                target: row.get(3)?,
                animated: row.get(4)?,
            })
        })
        .unwrap();
    let mut edges: Vec<Edge> = edge_iter.map(|edge| edge.unwrap()).collect();
    println!("DB edges: {:?}", edges);
    edges
}
