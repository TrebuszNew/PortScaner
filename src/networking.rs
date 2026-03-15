use std::io::{BufRead, Write};

fn main() {
    let handle = std::thread::spawn(|| {
        let server = std::net::TcpListener::bind("127.0.0.1:2137").unwrap();
        for stream in server.incoming() {
            let mut stream = stream.unwrap();

            let mut buff = std::io::BufReader::new(&stream);
            let mut user_data = String::new();
            buff.read_line(&mut user_data).unwrap();

            println!("Server: Dostaliśmy od usera: {}; wiadomość: \"{}\"", stream.peer_addr().unwrap(), user_data.trim());

            std::thread::sleep(std::time::Duration::from_secs(1));

            let data: String = format!("Dzięki za odpowiedź panie: {}. Miłego dnia 👋\n", stream.peer_addr().unwrap());
            stream.write(data.as_bytes()).unwrap();

        }
    });

    std::thread::sleep(std::time::Duration::from_secs(2));


    let mut user = std::net::TcpStream::connect("127.0.0.1:2137").unwrap();

    user.write("Witam serverze!\n".as_bytes()).unwrap();

    let mut buff = std::io::BufReader::new(user);
    let mut data = String::new();
    buff.read_line(&mut data).unwrap();

    println!("Klient: Otrzymaliśmy dane od serwera: \"{}\"", data.trim());

    handle.join().unwrap();

}