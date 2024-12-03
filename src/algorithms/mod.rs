use std::sync::Arc;
use tokio::sync::RwLock;

/// Trait defining the interface for load balancing algorithms
pub trait LoadBalancingAlgorithm: Send + Sync + Clone {
    /// Select the next server from the available servers
    fn next_server<'a>(&'a self, servers: &'a [String]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Option<String>> + Send + 'a>>;
}

/// Round-robin load balancing implementation
#[derive(Clone)]
pub struct RoundRobin {
    current: Arc<RwLock<usize>>,
}

impl RoundRobin {
    /// Create a new round-robin load balancer
    pub fn new() -> Self {
        Self {
            current: Arc::new(RwLock::new(0)),
        }
    }
}

impl LoadBalancingAlgorithm for RoundRobin {
    fn next_server<'a>(&'a self, servers: &'a [String]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Option<String>> + Send + 'a>> {
        Box::pin(async move {
            if servers.is_empty() {
                return None;
            }
            let mut current = self.current.write().await;
            *current = (*current + 1) % servers.len();
            Some(servers[*current].clone())
        })
    }
}
