use rust_challange::{Config, Monitor, ProviderTrait};
use std::sync::Arc;
#[derive(Debug)]
struct ColorfulMonitor;

impl ColorfulMonitor {
    fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Monitor for ColorfulMonitor {
    async fn on_start(&self, symbol: &str) {
        println!("[START] Provider: {}", symbol);
    }

    async fn on_finish(&self, symbol: &str, success: bool) {
        println!("[FINISH] is {} Provider: {}", success, symbol);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔══════════════════════════════════════════╗");
    println!("║   Rust Challenge - Price Provider Demo   ║");
    println!("╚══════════════════════════════════════════╝");

    #[cfg(feature = "a_provider")]
    let mock = async {
        println!("\n📦 Using A Provider feature");
        rust_challange::get_url_a_provider().await
    }
    .await;

    #[cfg(feature = "b_provider")]
    let mock = async {
        println!("\n📦 Using B Provider feature");
        rust_challange::get_url_b_provider().await
    }
    .await;

    println!("🌐 Mock URL: {}", mock.url);
    println!("{}", "".repeat(60));
    println!("{}", "─".repeat(60));

    demo_simple_fetch(&mock.url).await?;

    demo_concurrent_fetches(&mock.url).await?;

    demo_custom_monitor(&mock.url).await?;

    println!("\n✅ All demos completed successfully!");
    Ok(())
}

async fn demo_simple_fetch(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 DEMO 1: Simple Fetch");
    println!("{}", "─".repeat(60));

    let monitor = ColorfulMonitor::new();
    let config = Config::new(url, &monitor);
    let provider = config.build();

    println!("\nFetching price data...");
    let price_data = provider.fetch_price().await?;

    println!("\n📈 Price Data:");
    println!("   Symbol: {}", price_data.symbol);
    println!("   Price: {}", price_data.price);

    if let Some(name) = &price_data.name {
        println!("   Name: {}", name);
    }

    if let Some(description) = &price_data.description {
        println!("   Description: {}", description);
    }
    println!("{}", "".repeat(60));
    println!("{}", "─".repeat(60));
    Ok(())
}

async fn demo_concurrent_fetches(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 DEMO 2: Concurrent Fetches");

    println!("\nFetching 3 times concurrently...");
    println!("{}", "─".repeat(60));

    let mut handles = vec![];

    for i in 0..3 {
        let url_owned = url.to_string();
        let handle = tokio::spawn(async move {
            println!("\n[Fetch #{}] Starting...", i + 1);

            let monitor = ColorfulMonitor::new();
            let config = Config::new(&url_owned, &monitor);
            let provider = config.build();

            let result = provider.fetch_price().await;

            match result {
                Ok(data) => {
                    println!(
                        "[Fetch #{}] ✅ Success: {} = {}",
                        i + 1,
                        data.symbol,
                        data.price
                    );
                }
                Err(e) => {
                    println!("[Fetch #{}] ❌ Error: {}", i + 1, e);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }
    println!("{}", "".repeat(60));
    println!("{}", "─".repeat(60));
    Ok(())
}

async fn demo_custom_monitor(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 DEMO 3: Custom Monitor with History");
    println!("{}", "─".repeat(60));

    #[derive(Debug, Default)]
    struct HistoryMonitor {
        history: Arc<std::sync::Mutex<Vec<String>>>,
    }

    impl HistoryMonitor {
        fn new() -> Self {
            Self {
                history: Arc::new(std::sync::Mutex::new(Vec::new())),
            }
        }
    }

    #[async_trait::async_trait]
    impl Monitor for HistoryMonitor {
        async fn on_start(&self, provider: &str) {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let msg = format!("[{}] START - Provider: {}", timestamp, provider);
            self.history.lock().unwrap().push(msg.clone());
            println!("{}", msg);
        }

        async fn on_finish(&self, symbol: &str, success: bool) {
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let msg = format!(
                "[{}] FINISH is {} - Provider: {}",
                timestamp, success, symbol
            );
            self.history.lock().unwrap().push(msg.clone());
            println!("{}", msg);
        }
    }

    let monitor = HistoryMonitor::new();
    let history_clone = Arc::clone(&monitor.history);
    let config = Config::new(url, &monitor);
    let provider = config.build();

    println!("\nFetching with history monitor...");
    let price_data = provider.fetch_price().await?;

    println!("\n📈 Fetched Data:");
    println!("   Symbol: {}", price_data.symbol);
    println!("   Price: {}", price_data.price);

    println!("\n📋 Monitor History:");
    let history = history_clone.lock().unwrap();
    for (i, entry) in history.iter().enumerate() {
        println!("   {}. {}", i + 1, entry);
    }
    println!("{}", "".repeat(60));
    println!("{}", "─".repeat(60));
    Ok(())
}
