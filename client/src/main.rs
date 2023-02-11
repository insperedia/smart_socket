use rand::Rng;
use smart_hub::client::Client;
use smart_hub::transport::UdpTransport;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let transport = UdpTransport::new("localhost:2234".to_string(), "localhost:2233".to_string()).await;
    let client = Client { transport };
    let mut rng = rand::thread_rng();
    loop {
        let temp: i32 = rng.gen_range(-10..30);
        client
            .send(format!("therm1|||settemp#{temp}").as_str())
            .await
            .unwrap();
        thread::sleep(Duration::new(1, 0));
    }

    /*
    println!("Commands:\n");
    println!("    info - get device list");
    println!("    device_id|||command - run command on device");
    println!("            available commands: seton, setoff, info");
    println!("    exit - exit client ");

    loop {
        let mut cmd = String::new();
        let result = std::io::stdin().read_line(&mut cmd);
        match result {
            Ok(_) => {
                if cmd.eq("exit\n") {
                    break;
                }
                println!("Requesting... {}.", cmd);
                let result = client.send(cmd.as_str().trim());
                match result {
                    Ok(response) => {
                        println!("{}\n", response)
                    }
                    Err(error) => {
                        println!("Transport error: {}", error)
                    }
                }
            }
            Err(_) => {
                println!("Error reading command. Try again.\n")
            }
        }
    }

     */
}
