use rust_load_balancer::algorithms::{LoadBalancingAlgorithm, RoundRobin};
use rust_load_balancer::{balancer::LoadBalancer, generator::Generator, server::Server};

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::{time::timeout, time::Duration};

#[tokio::test]
async fn test_round_robin_no_timeout() {
    // Servers
    let server_port1 = 8001;
    let server_port2 = 8002;
    let load_balancer_port = 9000;

    let server1 = Server::new(server_port1, 100, 50);
    let server2 = Server::new(server_port2, 100, 50);

    let server1_handle = tokio::spawn(async move {
        server1.run().await;
    });

    let server2_handle = tokio::spawn(async move {
        server2.run().await;
    });

    // LB Start w/LocalHost
    let servers = vec![
        format!("127.0.0.1:{}", server_port1),
        format!("127.0.0.1:{}", server_port2),
    ];
    let load_balancer = LoadBalancer::new(load_balancer_port, servers);
    let load_balancer_handle = tokio::spawn(async move {
        load_balancer.run().await;
    });

    // Generator w/LB Port,
    let client_num = 10;
    let ratio = 0.7;
    let generator = Generator::new(
        &format!("http://127.0.0.1:{}", load_balancer_port),
        client_num,
        ratio,
    );

    let num_requests = 100;
    let generator_handle = tokio::spawn(async move {
        generator.run(num_requests).await;
    });

    // Result w/ Timeout issue (30 should be enough)
    let result = timeout(Duration::from_secs(30), generator_handle).await;

    // Abort handles
    server1_handle.abort();
    server2_handle.abort();
    load_balancer_handle.abort();

    assert!(result.is_ok(), "Test timed out before completion");
}

#[tokio::test]
async fn test_round_robin_empty_server_list() {
    let servers: Vec<String> = vec![];
    let round_robin = RoundRobin::new();
    let servers = Arc::new(RwLock::new(servers));

    let next_server = round_robin.next_server(&servers.read().await).await;

    // No server should be next
    assert!(next_server.is_none());
}
