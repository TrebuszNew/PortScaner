use colored::Colorize;
use std::{io::Write};
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio::sync::Semaphore;


fn main() {
    let address = get_address();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("błąd przy tworzeniu async runtime.");

    println!("Otwarte porty:");

    runtime.block_on(async {

        let mut set = tokio::task::JoinSet::new();

        let ip: IpAddr = address.parse().expect("Błędne IP");
        let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));

        for i in (1..=u16::MAX).rev() {
            let sem = Arc::clone(&semaphore);
            set.spawn(async move {
                check(ip, i, sem).await;
            });
        }

        while let Some(_) = set.join_next().await {}

    });
}

fn get_address() -> String {
    loop {
        clearscreen::clear().unwrap_or_else(|e| println!("Błąd w czyszczeniu konsoli: {e}"));
        print_logo();
        print!("IP address: ");
        std::io::stdout()
            .flush()
            .expect("Błąd flush");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("bład w czytaniu inputu usera!");
        let address = input.trim().to_string();

        if format!("{}:80", address).parse::<SocketAddr>().is_ok() {
            return address;
        };
    }
}

const MAX_CONCURRENT_REQUESTS: usize = 1000;

async fn check(address: IpAddr, port: u16, semaphore: Arc<Semaphore>) {
    // Pozyskanie "pozwolenia" z semafora
    let _permit = semaphore.acquire().await.unwrap();

    let socket_addr = SocketAddr::new(address, port);
    let connection = tokio::time::timeout(
        tokio::time::Duration::from_millis(1500), // Nieco dłuższy timeout bywa stabilniejszy
        tokio::net::TcpStream::connect(&socket_addr),
    )
    .await;

    if let Ok(Ok(_)) = connection {
        println!("🟢 Port {:5} jest {}", port, "OTWARTY".green().bold());
    }
}

fn print_logo() {
    println!(
        "{}",
        r" ____            _     ____                            
|  _ \ ___  _ __| |_  / ___|  ___ __ _ _ __   ___ _ __ 
| |_) / _ \| '__| __| \___ \ / __/ _` | '_ \ / _ \ '__|
|  __/ (_) | |  | |_   ___) | (_| (_| | | | |  __/ |   
|_|   \___/|_|   \__| |____/ \___\__,_|_| |_|\___|_|   
"
        .blue().bold()
    );
}
