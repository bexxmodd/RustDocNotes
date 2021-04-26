use std::thread;
use std::time::Duration;
use std::net::TcpListener;

fn open_socket_for_five_seconds() {
        let _listener = TcpListener::bing("127.0.0.1:5000").unwrap();
        thread::sleep(Duration::from_secs(5));
}

struct Noisy {
        id: i32,
}

impl Drop for Noisy {
        fn drop(&mut self) {
                println!("Noisy number {} going out of scope!", self.id);
        }
}

fn main() {
        let _n1 = Noisy { id: 1 };
        let _n2 = Noisy { id: 2};
        println!("End of Noisy");
        open_socket_for_five_seconds();
        println!("Bacj in main");
        thread::sleep(Duration::from_secs(5));
}


