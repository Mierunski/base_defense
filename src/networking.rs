use std::{
    net::{SocketAddr, UdpSocket},
    time::SystemTime,
};

use bevy::prelude::*;
use bevy_renet::{
    renet::{
        ClientAuthentication, RenetClient, RenetConnectionConfig, RenetServer,
        ServerAuthentication, ServerConfig,
    },
    RenetClientPlugin, RenetServerPlugin,
};
use renet_visualizer::{RenetClientVisualizer, RenetVisualizerStyle};

const PROTOCOL_ID: u64 = 0;

// Helper struct to pass an username in the user data
struct Username(String);
pub struct NetworkingPlugin {
    exec_type: String,
    server_addr: SocketAddr,
    username: Username,
}

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        match self.exec_type.as_str() {
            "client" => self.start_client(app),
            "server" => self.start_server(app),

            _ => {
                println!("Invalid argument, first one must be \"client\" or \"server\".");
            }
        }
    }
}

impl NetworkingPlugin {
    pub fn new(args: &Vec<String>) -> NetworkingPlugin {
        NetworkingPlugin {
            exec_type: args[1].clone(),
            server_addr: format!("127.0.0.1:{}", args[2]).parse().unwrap(),
            username: Username(args[3].clone()),
        }
    }

    fn start_server(&self, app: &mut App) {
        app.add_plugin(RenetServerPlugin);

        let socket = UdpSocket::bind(self.server_addr).unwrap();
        let connection_config = RenetConnectionConfig::default();
        let server_config = ServerConfig::new(
            64,
            PROTOCOL_ID,
            self.server_addr,
            ServerAuthentication::Unsecure,
        );
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let server =
            RenetServer::new(current_time, server_config, connection_config, socket).unwrap();
        app.insert_resource(server);

        app.add_system(Server::send_message_system);
        app.add_system(Server::receive_message_system);
        app.add_system(Server::handle_events_system);
    }

    fn start_client(&self, app: &mut App) {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let connection_config = RenetConnectionConfig::default();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let client_id = current_time.as_millis() as u64;
        let authentication = ClientAuthentication::Unsecure {
            client_id,
            protocol_id: PROTOCOL_ID,
            server_addr: self.server_addr,
            user_data: None,
        };
        let client = RenetClient::new(
            current_time,
            socket,
            client_id,
            connection_config,
            authentication,
        )
        .unwrap();
        app.add_plugin(RenetClientPlugin);
        app.insert_resource(client);

        app.add_system(Client::send_message_system);
        app.add_system(Client::receive_message_system);

        app.insert_resource(RenetClientVisualizer::<200>::new(
            RenetVisualizerStyle::default(),
        ));
    }
}
mod Server {
    use bevy::prelude::*;
    use bevy_renet::renet::{RenetServer, ServerEvent};

    pub fn send_message_system(mut server: ResMut<RenetServer>) {
        let channel_id = 0;
        // Send a text message for all clients
        server.broadcast_message(channel_id, "server message".as_bytes().to_vec());
    }

    pub fn receive_message_system(mut server: ResMut<RenetServer>) {
        let channel_id = 0;
        // Send a text message for all clients
        for client_id in server.clients_id().into_iter() {
            while let Some(message) = server.receive_message(client_id, channel_id) {
                // Handle received message
                info!(
                    "Received message: {}",
                    std::str::from_utf8(&message).unwrap()
                );
            }
        }
    }

    pub fn handle_events_system(
        mut server: ResMut<RenetServer>,
        mut server_events: EventReader<ServerEvent>,
    ) {
        while let Some(event) = server.get_event() {
            for event in server_events.iter() {
                match event {
                    ServerEvent::ClientConnected(id, user_data) => {
                        println!("Client {} connected", id);
                    }
                    ServerEvent::ClientDisconnected(id) => {
                        println!("Client {} disconnected", id);
                    }
                }
            }
        }
    }
}

mod Client {
    use bevy::prelude::*;
    use bevy_egui::EguiContext;
    use bevy_renet::renet::RenetClient;
    use renet_visualizer::RenetClientVisualizer;

    pub fn send_message_system(mut client: ResMut<RenetClient>) {
        let channel_id = 0;
        // Send a text message to the server
        client.send_message(
            channel_id,
            "client message plpl opaskdopkas pkdopas kpdosk"
                .as_bytes()
                .to_vec(),
        );
    }

    pub fn receive_message_system(
        mut client: ResMut<RenetClient>,
        mut egui_context: ResMut<EguiContext>,
        mut visualizer: ResMut<RenetClientVisualizer<200>>,
    ) {
        let channel_id = 0;
        while let Some(message) = client.receive_message(channel_id) {
            info!(
                "Received message: {}",
                std::str::from_utf8(&message).unwrap()
            );
        }

        visualizer.add_network_info(client.network_info());

        visualizer.show_window(egui_context.ctx_mut());
    }
}
