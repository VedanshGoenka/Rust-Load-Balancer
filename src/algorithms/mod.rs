use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Trait defining the interface for load balancing algorithms
pub trait LoadBalancingAlgorithm: Send + Sync + Clone {
    /// Select the next server from the available servers
    fn next_server<'a>(
        &'a self,
        servers: &'a [String],
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Option<String>> + Send + 'a>>;

    /// Track when a connection starts
    fn connection_started(
        &self,
        server: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>;

    /// Track when a connection ends
    fn connection_ended(
        &self,
        server: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>;

    /// Get server metrics
    fn get_metrics(
        &self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = HashMap<String, String>> + Send + 'static>,
    >;
}

/// Available load balancing algorithms
#[derive(Clone)]
pub enum Algorithm {
    RoundRobin(RoundRobin),
    LeastConnections(LeastConnections),
    WeightedRoundRobin(WeightedRoundRobin),
    IpHash(IpHash),
}

impl Algorithm {
    pub fn new(algo_type: &str, weights: Option<HashMap<String, u32>>) -> Self {
        match algo_type {
            "round-robin" => Algorithm::RoundRobin(RoundRobin::new()),
            "least-connections" => Algorithm::LeastConnections(LeastConnections::new()),
            "weighted-round-robin" => {
                Algorithm::WeightedRoundRobin(WeightedRoundRobin::new(weights))
            }
            "ip-hash" => Algorithm::IpHash(IpHash::new()),
            _ => Algorithm::RoundRobin(RoundRobin::new()), // Default to round-robin
        }
    }
}

impl LoadBalancingAlgorithm for Algorithm {
    fn next_server<'a>(
        &'a self,
        servers: &'a [String],
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Option<String>> + Send + 'a>> {
        match self {
            Algorithm::RoundRobin(rr) => rr.next_server(servers),
            Algorithm::LeastConnections(lc) => lc.next_server(servers),
            Algorithm::WeightedRoundRobin(wrr) => wrr.next_server(servers),
            Algorithm::IpHash(ih) => ih.next_server(servers),
        }
    }

    fn connection_started(
        &self,
        server: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
        let server = server.to_string();
        match self {
            Algorithm::RoundRobin(_) => Box::pin(async {}),
            Algorithm::LeastConnections(lc) => {
                let lc = lc.clone();
                Box::pin(async move { lc.connection_started(&server).await })
            }
            Algorithm::WeightedRoundRobin(_) => Box::pin(async {}),
            Algorithm::IpHash(_) => Box::pin(async {}),
        }
    }

    fn connection_ended(
        &self,
        server: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
        let server = server.to_string();
        match self {
            Algorithm::RoundRobin(_) => Box::pin(async {}),
            Algorithm::LeastConnections(lc) => {
                let lc = lc.clone();
                Box::pin(async move { lc.connection_ended(&server).await })
            }
            Algorithm::WeightedRoundRobin(_) => Box::pin(async {}),
            Algorithm::IpHash(_) => Box::pin(async {}),
        }
    }

    fn get_metrics(
        &self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = HashMap<String, String>> + Send + 'static>,
    > {
        match self {
            Algorithm::RoundRobin(rr) => {
                let rr = rr.clone();
                Box::pin(async move { rr.get_metrics().await })
            }
            Algorithm::LeastConnections(lc) => {
                let lc = lc.clone();
                Box::pin(async move { lc.get_metrics().await })
            }
            Algorithm::WeightedRoundRobin(wrr) => {
                let wrr = wrr.clone();
                Box::pin(async move { wrr.get_metrics().await })
            }
            Algorithm::IpHash(ih) => {
                let ih = ih.clone();
                Box::pin(async move { ih.get_metrics().await })
            }
        }
    }
}

/// Round-robin load balancing implementation
#[derive(Clone)]
pub struct RoundRobin {
    current: Arc<RwLock<usize>>,
    requests_served: Arc<RwLock<HashMap<String, usize>>>,
}

impl RoundRobin {
    pub fn new() -> Self {
        Self {
            current: Arc::new(RwLock::new(0)),
            requests_served: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn record_request(&self, server: &str) {
        let mut requests = self.requests_served.write().await;
        *requests.entry(server.to_string()).or_insert(0) += 1;
    }
}

impl LoadBalancingAlgorithm for RoundRobin {
    fn next_server<'a>(
        &'a self,
        servers: &'a [String],
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Option<String>> + Send + 'a>> {
        Box::pin(async move {
            if servers.is_empty() {
                return None;
            }
            let mut current = self.current.write().await;
            *current = (*current + 1) % servers.len();
            let server = servers[*current].clone();
            self.record_request(&server).await;
            Some(server)
        })
    }

    fn connection_started(
        &self,
        _: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
        Box::pin(async {})
    }

    fn connection_ended(
        &self,
        _: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
        Box::pin(async {})
    }

    fn get_metrics(
        &self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = HashMap<String, String>> + Send + 'static>,
    > {
        let this = self.clone();
        Box::pin(async move {
            let requests = this.requests_served.read().await;
            let total_requests: usize = requests.values().sum();

            requests
                .iter()
                .map(|(server, count)| {
                    let percentage = if total_requests > 0 {
                        (*count as f64 / total_requests as f64) * 100.0
                    } else {
                        0.0
                    };
                    (
                        server.clone(),
                        format!("Requests: {}, Distribution: {:.1}%", count, percentage),
                    )
                })
                .collect()
        })
    }
}

/// Least connections implementation
#[derive(Clone)]
pub struct LeastConnections {
    connections: Arc<RwLock<HashMap<String, usize>>>,
    total_requests: Arc<RwLock<HashMap<String, usize>>>,
    successful_requests: Arc<RwLock<HashMap<String, usize>>>,
}

impl LeastConnections {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            total_requests: Arc::new(RwLock::new(HashMap::new())),
            successful_requests: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn connection_started(&self, server: &str) {
        let mut connections = self.connections.write().await;
        let mut total = self.total_requests.write().await;
        *connections.entry(server.to_string()).or_insert(0) += 1;
        *total.entry(server.to_string()).or_insert(0) += 1;
    }

    pub async fn connection_ended(&self, server: &str) {
        let mut connections = self.connections.write().await;
        let mut successful = self.successful_requests.write().await;
        if let Some(count) = connections.get_mut(server) {
            if *count > 0 {
                *count -= 1;
                *successful.entry(server.to_string()).or_insert(0) += 1;
            }
        }
    }

    pub async fn get_metrics(&self) -> HashMap<String, String> {
        let connections = self.connections.read().await;
        let total = self.total_requests.read().await;
        let successful = self.successful_requests.read().await;

        let mut metrics = HashMap::new();
        for (server, conn) in connections.iter() {
            let total_reqs = total.get(server).unwrap_or(&0);
            let success_reqs = successful.get(server).unwrap_or(&0);
            let success_rate = if *total_reqs > 0 {
                (*success_reqs as f64 / *total_reqs as f64) * 100.0
            } else {
                0.0
            };

            metrics.insert(
                server.clone(),
                format!(
                    "Active: {}, Total: {}, Success: {}, Rate: {:.1}%",
                    conn, total_reqs, success_reqs, success_rate
                ),
            );
        }
        metrics
    }
}

impl LoadBalancingAlgorithm for LeastConnections {
    fn next_server<'a>(
        &'a self,
        servers: &'a [String],
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Option<String>> + Send + 'a>> {
        Box::pin(async move {
            if servers.is_empty() {
                return None;
            }
            let connections = self.connections.read().await;
            servers
                .iter()
                .min_by_key(|server| connections.get(*server).unwrap_or(&0))
                .map(|s| s.clone())
        })
    }

    fn connection_started(
        &self,
        server: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
        let server = server.to_string();
        let this = self.clone();
        Box::pin(async move {
            this.connection_started(&server).await;
        })
    }

    fn connection_ended(
        &self,
        server: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
        let server = server.to_string();
        let this = self.clone();
        Box::pin(async move {
            this.connection_ended(&server).await;
        })
    }

    fn get_metrics(
        &self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = HashMap<String, String>> + Send + 'static>,
    > {
        let this = self.clone();
        Box::pin(async move {
            let connections = this.connections.read().await;
            connections
                .iter()
                .map(|(k, v)| (k.clone(), format!("Active connections: {}", v)))
                .collect()
        })
    }
}

/// Weighted round-robin implementation with randomized weights
#[derive(Clone)]
pub struct WeightedRoundRobin {
    current: Arc<RwLock<usize>>,
    weights: Arc<RwLock<HashMap<String, u32>>>,
    requests_served: Arc<RwLock<HashMap<String, usize>>>,
}

impl WeightedRoundRobin {
    pub fn new(weights: Option<HashMap<String, u32>>) -> Self {
        Self {
            current: Arc::new(RwLock::new(0)),
            weights: Arc::new(RwLock::new(weights.unwrap_or_default())),
            requests_served: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_metrics(&self) -> HashMap<String, String> {
        let weights = self.weights.read().await;
        let requests = self.requests_served.read().await;
        let total_requests: usize = requests.values().sum();

        weights
            .iter()
            .map(|(server, weight)| {
                let served = requests.get(server).unwrap_or(&0);
                let percentage = if total_requests > 0 {
                    (*served as f64 / total_requests as f64) * 100.0
                } else {
                    0.0
                };
                (
                    server.clone(),
                    format!(
                        "Weight: {}, Requests: {}, Distribution: {:.1}%",
                        weight, served, percentage
                    ),
                )
            })
            .collect()
    }

    async fn ensure_weights(&self, servers: &[String]) {
        let mut weights = self.weights.write().await;
        let mut rng = thread_rng();

        for server in servers {
            if !weights.contains_key(server) {
                weights.insert(server.clone(), rng.gen_range(1..=10));
            }
        }
    }

    async fn record_request(&self, server: &str) {
        let mut requests = self.requests_served.write().await;
        *requests.entry(server.to_string()).or_insert(0) += 1;
    }
}

impl LoadBalancingAlgorithm for WeightedRoundRobin {
    fn next_server<'a>(
        &'a self,
        servers: &'a [String],
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Option<String>> + Send + 'a>> {
        Box::pin(async move {
            if servers.is_empty() {
                return None;
            }

            self.ensure_weights(servers).await;

            let weights = self.weights.read().await;
            let total_weight: u32 = servers.iter().map(|s| weights.get(s).unwrap_or(&1)).sum();

            let mut current = self.current.write().await;
            *current = (*current + 1) % (total_weight as usize);

            let mut accumulator = 0;
            for server in servers {
                accumulator += weights.get(server).unwrap_or(&1);
                if (*current as u32) < accumulator {
                    self.record_request(server).await;
                    return Some(server.clone());
                }
            }

            self.record_request(&servers[0]).await;
            Some(servers[0].clone())
        })
    }

    fn connection_started(
        &self,
        _: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
        Box::pin(async {})
    }

    fn connection_ended(
        &self,
        _: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
        Box::pin(async {})
    }

    fn get_metrics(
        &self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = HashMap<String, String>> + Send + 'static>,
    > {
        let this = self.clone();
        Box::pin(async move {
            let weights = this.weights.read().await;
            weights
                .iter()
                .map(|(k, v)| (k.clone(), format!("Weight: {}", v)))
                .collect()
        })
    }
}

/// IP hash implementation
#[derive(Clone)]
pub struct IpHash {
    requests_served: Arc<RwLock<HashMap<String, usize>>>,
    ip_distribution: Arc<RwLock<HashMap<String, String>>>,
}

impl IpHash {
    pub fn new() -> Self {
        Self {
            requests_served: Arc::new(RwLock::new(HashMap::new())),
            ip_distribution: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn hash(ip: &str) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        ip.hash(&mut hasher);
        hasher.finish()
    }

    async fn record_request(&self, server: &str, ip: &str) {
        let mut requests = self.requests_served.write().await;
        let mut dist = self.ip_distribution.write().await;
        *requests.entry(server.to_string()).or_insert(0) += 1;
        dist.insert(ip.to_string(), server.to_string());
    }
}

impl LoadBalancingAlgorithm for IpHash {
    fn next_server<'a>(
        &'a self,
        servers: &'a [String],
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Option<String>> + Send + 'a>> {
        Box::pin(async move {
            if servers.is_empty() {
                return None;
            }
            // Using different IPs for testing distribution
            let test_ips = ["192.168.1.1", "10.0.0.1", "172.16.0.1"];
            let ip = test_ips[rand::thread_rng().gen_range(0..test_ips.len())];
            let hash = Self::hash(ip);
            let index = (hash % servers.len() as u64) as usize;
            let server = servers[index].clone();
            self.record_request(&server, ip).await;
            Some(server)
        })
    }

    fn connection_started(
        &self,
        _: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
        Box::pin(async {})
    }

    fn connection_ended(
        &self,
        _: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
        Box::pin(async {})
    }

    fn get_metrics(
        &self,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = HashMap<String, String>> + Send + 'static>,
    > {
        let this = self.clone();
        Box::pin(async move {
            let requests = this.requests_served.read().await;
            let dist = this.ip_distribution.read().await;
            let total_requests: usize = requests.values().sum();

            let mut metrics = HashMap::new();
            for (server, count) in requests.iter() {
                let percentage = if total_requests > 0 {
                    (*count as f64 / total_requests as f64) * 100.0
                } else {
                    0.0
                };

                let ip_mappings: Vec<String> = dist
                    .iter()
                    .filter(|(_, s)| *s == server)
                    .map(|(ip, _)| ip.clone())
                    .collect();

                metrics.insert(
                    server.clone(),
                    format!(
                        "Requests: {}, Distribution: {:.1}%, IPs: {}",
                        count,
                        percentage,
                        ip_mappings.join(", ")
                    ),
                );
            }
            metrics
        })
    }
}
