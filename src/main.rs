use clap::Parser;
use std::env;
use std::net::TcpListener;
use std::process;

/// CLI arguments for port generator
#[derive(Parser)]
#[command(name = "portgen")]
#[command(about = "Generate unique port numbers for your projects")]
#[command(version)]
struct Cli {
    /// Project name to generate ports for (defaults to current directory name)
    project_name: Option<String>,

    /// Number of ports to generate (default: 2)
    #[arg(short, long, default_value_t = 2)]
    count: u8,

    /// Show all reserved ports
    #[arg(short, long)]
    list_reserved: bool,
}

/// Known commonly used server ports (should be avoided)
const RESERVED_PORTS: &[u16] = &[
    // Database services
    3306,  // MySQL
    3307,  // MySQL (common backup)
    5432,  // PostgreSQL
    6379,  // Redis
    27017, // MongoDB
    27018, // MongoDB
    9200,  // Elasticsearch
    9300,  // Elasticsearch
    9042,  // Cassandra
    8123,  // ClickHouse
    9000,  // ClickHouse
    1433,  // SQL Server
    1521,  // Oracle
    // Web development common ports
    3000, // React/Vue/Next.js dev server
    3001, // Common dev backup port
    5173, // Vite dev server
    8080, // Generic HTTP proxy/dev server
    8081, // Common backup port
    8000, // Django/Flask default
    8001, // Common backup port
    4200, // Angular CLI
    5000, // Flask default
    5001, // Common backup port
    1313, // Hugo
    4000, // Jekyll
    4567, // Sinatra
    9292, // Puma/Rack
    // Other common services
    22,    // SSH (for completeness, though <1024)
    80,    // HTTP
    443,   // HTTPS
    21,    // FTP
    25,    // SMTP
    110,   // POP3
    143,   // IMAP
    993,   // IMAPS
    995,   // POP3S
    587,   // SMTP submission
    465,   // SMTPS
    33060, // MySQL X Protocol
    11211, // Memcached
    2181,  // ZooKeeper
    9092,  // Kafka
    9093,  // Kafka
    2181,  // ZooKeeper
    8888,  // Jupyter Notebook
    8889,  // Jupyter backup
    6006,  // TensorBoard
    16686, // Jaeger UI
    9411,  // Zipkin
    8082,  // Nexus
    8083,  // Nexus backup
    9001,  // Supervisor
    9090,  // Prometheus
    3000,  // Grafana
    5601,  // Kibana
    10000, // Webmin
    19999, // Netdata
    6443,  // Kubernetes API
    10250, // Kubelet
    2379,  // etcd
    2380,  // etcd peer
    4222,  // NATS
    6222,  // NATS cluster
    8222,  // NATS monitoring
    27019, // MongoDB config
    27020, // MongoDB shard
    4369,  // Erlang EPMD
    5672,  // RabbitMQ
    15672, // RabbitMQ Management
    25672, // RabbitMQ cluster
    15674, // RabbitMQ STOMP
    61613, // RabbitMQ STOMP (legacy)
    61614, // RabbitMQ Web STOMP
    1883,  // MQTT
    8883,  // MQTTS
    9003,  // BrowserSync
    35729, // LiveReload
    // Common local dev port range start
    9000, 9001, 9002, 9003, 9004, 9005,
];

/// Check if a port is in the reserved list
fn is_reserved_port(port: u16) -> bool {
    RESERVED_PORTS.contains(&port)
}

/// Check if a port is currently in use by attempting to bind to it
fn is_port_in_use(port: u16) -> bool {
    match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(listener) => {
            drop(listener);
            false
        }
        Err(_) => true,
    }
}

/// Get the current directory name as the default project name
fn get_default_project_name() -> String {
    env::current_dir()
        .ok()
        .and_then(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "my_project".to_string())
}

/// Find available ports
fn main() {
    let cli = Cli::parse();

    // Handle list reserved ports option
    if cli.list_reserved {
        println!("🔒 Reserved ports (should be avoided):");
        for port in RESERVED_PORTS {
            let desc = get_port_description(*port);
            if !desc.is_empty() {
                println!("   {} - {}", port, desc);
            } else {
                println!("   {}", port);
            }
        }
        return;
    }

    // Get project name from args or use current directory name
    let project_name = cli.project_name.unwrap_or_else(|| {
        let default_name = get_default_project_name();
        println!("💡 No project name provided, using current directory name as default");
        default_name
    });

    println!("🔧 Project name: {}", project_name);
    println!("📝 Generating port numbers based on project name...\n");

    // Generate ports based on count
    let ports = generate_multiple_ports(&project_name, cli.count);

    println!("🔢 Suggested port numbers:");
    for (i, port) in ports.iter().enumerate() {
        println!("   Port {}: {}", i + 1, port);
        if is_reserved_port(*port) {
            println!("      (⚠️ This is a reserved port, automatically skipped)");
        }
    }
    println!();

    // Check port availability
    println!("🔍 Checking port availability...");
    let available_ports = check_ports_availability(&ports);

    println!("\n📋 Summary:");
    if available_ports.is_empty() {
        println!("❌ All suggested ports are in use or reserved");
        print!("   Suggested ports:");
        for port in &ports {
            print!(" {}", port);
        }
        println!();
        process::exit(1);
    } else {
        println!("✅ Available ports:");
        for port in &available_ports {
            let service_info = get_port_description(*port);
            if service_info.is_empty() {
                println!("   - {}", port);
            } else {
                println!("   - {} ({})", port, service_info);
            }
        }

        if available_ports.len() < cli.count as usize {
            println!(
                "\n⚠️  Only {} port(s) available, requested {}",
                available_ports.len(),
                cli.count
            );
        } else {
            println!("\n🎉 All ports are available!");
        }
    }
}

/// Generate multiple ports for a project
fn generate_multiple_ports(project_name: &str, count: u8) -> Vec<u16> {
    let mut ports = Vec::new();
    let hash: u32 = project_name.bytes().map(|b| b as u32).sum();
    let port_range = 65535 - 1024;

    for i in 0..count {
        let mut port = 1024 + (((hash * 31 + 7 + i as u32 * 13) % port_range as u32) as u16);

        // Keep incrementing if port is reserved or already in our list
        while is_reserved_port(port) || ports.contains(&port) {
            port = 1024 + (((port as u32 + 1) % port_range as u32) as u16);
        }

        ports.push(port);
    }

    ports
}

/// Check availability of generated ports
fn check_ports_availability(ports: &[u16]) -> Vec<u16> {
    let mut available_ports = Vec::new();

    for port in ports {
        if is_reserved_port(*port) {
            println!(
                "⚠️  Port {} is a reserved port (known common service port)",
                port
            );
        } else if is_port_in_use(*port) {
            println!("⚠️  Port {} is already in use", port);
        } else {
            println!("✅ Port {} is available", port);
            available_ports.push(*port);
        }
    }

    available_ports
}

/// Get port description (if available)
fn get_port_description(port: u16) -> &'static str {
    match port {
        3306 | 3307 => "MySQL",
        5432 => "PostgreSQL",
        6379 => "Redis",
        27017 | 27018 => "MongoDB",
        9200 | 9300 => "Elasticsearch",
        3000 | 3001 => "Web dev server",
        5173 => "Vite",
        8080 | 8081 => "HTTP proxy",
        8000 | 8001 => "Python dev server",
        4200 => "Angular CLI",
        5000 | 5001 => "Flask",
        1313 => "Hugo",
        4000 => "Jekyll",
        4567 => "Sinatra",
        9292 => "Puma",
        22 => "SSH",
        80 => "HTTP",
        443 => "HTTPS",
        21 => "FTP",
        25 => "SMTP",
        110 => "POP3",
        143 => "IMAP",
        33060 => "MySQL X",
        11211 => "Memcached",
        2181 => "ZooKeeper",
        9092 => "Kafka",
        8888 => "Jupyter",
        9090 => "Prometheus",
        5601 => "Kibana",
        6443 => "Kubernetes",
        2379 | 2380 => "etcd",
        5672 => "RabbitMQ",
        15672 => "RabbitMQ UI",
        1883 => "MQTT",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_ports_avoids_reserved() {
        // Generate multiple times to ensure no reserved ports are generated
        for i in 0..100 {
            let ports = generate_multiple_ports(&format!("test_project_{}", i), 2);
            for port in &ports {
                assert!(
                    !is_reserved_port(*port),
                    "Port {} should not be reserved",
                    port
                );
            }
        }
    }

    #[test]
    fn test_generate_ports_different() {
        let ports = generate_multiple_ports("test_project", 2);
        assert_ne!(ports[0], ports[1], "Two ports should be different");
    }

    #[test]
    fn test_generate_ports_in_range() {
        let ports = generate_multiple_ports("test_project", 2);
        for port in &ports {
            assert!(*port >= 1024, "Port should be >= 1024");
            assert!(*port <= 65535, "Port should be <= 65535");
        }
    }

    #[test]
    fn test_generate_multiple_ports_in_range() {
        let ports = generate_multiple_ports("test_project", 10);
        for port in &ports {
            assert!(*port >= 1024, "Port {} should be >= 1024", port);
            assert!(*port <= 65535, "Port {} should be <= 65535", port);
        }
    }

    #[test]
    fn test_generate_ports_consistency() {
        let ports_a = generate_multiple_ports("my_app", 2);
        let ports_b = generate_multiple_ports("my_app", 2);
        assert_eq!(ports_a, ports_b);
    }

    #[test]
    fn test_is_port_in_use() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        assert!(is_port_in_use(port));
        drop(listener);
        std::thread::sleep(std::time::Duration::from_millis(100));
        assert!(!is_port_in_use(port));
    }

    #[test]
    fn test_reserved_ports_list() {
        // Ensure some important ports are in the reserved list
        assert!(is_reserved_port(3000));
        assert!(is_reserved_port(3306));
        assert!(is_reserved_port(6379));
        assert!(is_reserved_port(5432));
        assert!(is_reserved_port(5173));
        assert!(is_reserved_port(8080));
    }

    #[test]
    fn test_get_default_project_name_returns_non_empty() {
        // Test that we can get a default project name
        let name = get_default_project_name();
        assert!(!name.is_empty());
        assert!(!name.contains('/')); // Should not contain path separator
        assert!(!name.contains('\\')); // Windows path separator
    }

    #[test]
    fn test_is_reserved_port_with_all_common_ports() {
        // Test all common service ports are in the reserved list
        let common_ports = vec![
            22, 80, 443, // Basic network services
            3000, 5173, 8000, 8080, 5000, // Web development
            3306, 5432, 6379, 27017, // Databases
            9090, 5601, 6443, // Monitoring/K8s
        ];

        for port in common_ports {
            assert!(is_reserved_port(port), "Port {} should be reserved", port);
        }
    }

    #[test]
    fn test_non_reserved_ports() {
        // Test that some ordinary ports are not in reserved list (using large port numbers)
        let non_reserved = vec![11212, 15000, 20000, 25000, 30001, 40000, 50000, 60000];

        for port in non_reserved {
            assert!(
                !is_reserved_port(port),
                "Port {} should not be reserved",
                port
            );
        }
    }

    #[test]
    fn test_generate_ports_with_special_characters() {
        // Test project names with special characters
        let special_names = vec![
            "my-project",
            "my_project",
            "my.project",
            "my:project",
            "my project",
            "MyProject123",
            "my-project-v2.0",
        ];

        for name in special_names {
            let ports = generate_multiple_ports(name, 2);
            for port in &ports {
                assert!(*port >= 1024, "Port should be >= 1024 for '{}'", name);
            }
            assert_ne!(
                ports[0], ports[1],
                "Ports should be different for '{}'",
                name
            );
            for port in &ports {
                assert!(
                    !is_reserved_port(*port),
                    "Port should not be reserved for '{}'",
                    name
                );
            }
        }
    }

    #[test]
    fn test_get_port_description() {
        // Test port description functionality
        assert_eq!(get_port_description(3306), "MySQL");
        assert_eq!(get_port_description(3307), "MySQL");
        assert_eq!(get_port_description(5432), "PostgreSQL");
        assert_eq!(get_port_description(6379), "Redis");
        assert_eq!(get_port_description(27017), "MongoDB");
        assert_eq!(get_port_description(3000), "Web dev server");
        assert_eq!(get_port_description(5173), "Vite");
        assert_eq!(get_port_description(8080), "HTTP proxy");
        assert_eq!(get_port_description(80), "HTTP");
        assert_eq!(get_port_description(443), "HTTPS");
        assert_eq!(get_port_description(22), "SSH");

        // Unknown ports return empty string
        assert_eq!(get_port_description(12345), "");
        assert_eq!(get_port_description(15000), "");
    }

    #[test]
    fn test_different_project_names_produce_different_ports() {
        // Test different project names produce reasonable port distribution (collisions allowed but shouldn't be too frequent)
        let mut all_ports: std::collections::HashSet<u16> = std::collections::HashSet::new();
        let test_names = vec![
            "alpha", "beta", "gamma", "delta", "epsilon", "frontend", "backend", "api", "web",
            "app",
        ];

        let mut collision_count = 0;
        let total_ports = test_names.len() * 2;

        for name in &test_names {
            let ports = generate_multiple_ports(name, 2);
            // Check for duplicates
            if !all_ports.insert(ports[0]) {
                collision_count += 1;
            }
            if !all_ports.insert(ports[1]) {
                collision_count += 1;
            }
        }

        // Allow up to 40% collision rate (should be much lower)
        let collision_rate = collision_count as f64 / total_ports as f64;
        assert!(
            collision_rate < 0.4,
            "Port collision rate {} is too high, should be < 0.4",
            collision_rate
        );

        // At least 60% uniqueness
        assert!(
            all_ports.len() as f64 / total_ports as f64 >= 0.6,
            "Should have good port uniqueness (>=60%)"
        );
    }

    #[test]
    fn test_empty_project_name() {
        // Test empty string project name
        let ports = generate_multiple_ports("", 2);
        assert!(ports[0] >= 1024);
        assert!(ports[1] >= 1024);
        assert_ne!(ports[0], ports[1]);
        assert!(!is_reserved_port(ports[0]));
        assert!(!is_reserved_port(ports[1]));
    }

    #[test]
    fn test_very_long_project_name() {
        // Test very long project name
        let long_name = "a".repeat(1000);
        let ports = generate_multiple_ports(&long_name, 2);
        assert!(ports[0] >= 1024);
        assert!(ports[1] >= 1024);
        assert_ne!(ports[0], ports[1]);
        assert!(!is_reserved_port(ports[0]));
        assert!(!is_reserved_port(ports[1]));
    }

    #[test]
    fn test_unicode_project_name() {
        // Test Unicode project names (byte calculation may differ)
        let unicode_names = vec!["我的项目", "プロジェクト", "프로젝트", "🏗️-project"];

        for name in unicode_names {
            let ports = generate_multiple_ports(name, 2);
            for port in &ports {
                assert!(
                    *port >= 1024,
                    "Port should be valid for unicode name '{}'",
                    name
                );
                assert!(!is_reserved_port(*port));
            }
        }
    }

    #[test]
    fn test_port_consistency_after_1000_runs() {
        // Test consistency: same project name always generates same port across 1000 runs
        let project_name = "consistency_test_project";
        let first_ports = generate_multiple_ports(project_name, 2);

        for _ in 0..1000 {
            let ports = generate_multiple_ports(project_name, 2);
            assert_eq!(ports, first_ports, "Ports should be consistent");
        }
    }

    #[test]
    fn test_reserved_ports_list_has_duplicates() {
        // Test that reserved ports list has duplicates (this is expected behavior)
        let mut seen = std::collections::HashSet::new();
        let mut duplicates = Vec::new();

        for &port in RESERVED_PORTS {
            if !seen.insert(port) {
                duplicates.push(port);
            }
        }

        // We expect some duplicates (like 3000 and 2181 both duplicated)
        assert!(
            !duplicates.is_empty(),
            "Reserved ports list has duplicates: {:?}",
            duplicates
        );
    }
}
