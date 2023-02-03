use smart_hub::client::Client;
use smart_hub::transport::TcpTransport;

fn main() {
    let transport = TcpTransport::new("localhost:2233".to_string());

    let client = Client {
        transport,
    };
    
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
               let result =  client.send(cmd.as_str());
                match result {
                    Ok(response) => {println!("{}\n", response)}
                    Err(error) => { println!("Transport error: {}", error )}
                }
            }
            Err(_) => { println!("Error reading command. Try again.\n")}
        }
    }




}
