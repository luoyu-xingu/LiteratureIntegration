use neo4rs::Graph;

#[derive(Debug, Clone)]
pub struct Config {
    pub neo4j_uri: String,
    pub neo4j_user: String,
    pub neo4j_password: String,
    pub server_host: String,
    pub server_port: u16,
    pub cors_origin: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            neo4j_uri: std::env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".into()),
            neo4j_user: std::env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".into()),
            neo4j_password: std::env::var("NEO4J_PASSWORD").unwrap_or_else(|_| "password".into()),
            server_host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .unwrap(),
            cors_origin: std::env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".into()),
        }
    }
}

pub async fn create_neo4j_pool(config: &Config) -> Result<Graph, neo4rs::Error> {
    let config = neo4rs::ConfigBuilder::default()
        .uri(&config.neo4j_uri)
        .user(&config.neo4j_user)
        .password(&config.neo4j_password)
        .max_connections(50)
        .fetch_size(20000)
        .build()?;
    Graph::connect(config).await
}
