mod app;
pub mod state;

use std::sync::{atomic::AtomicPtr, Arc};

use app::TemplateApp;
use clock_synchronization::quartz_clock::QuartzUtcClock;
use eframe::egui;
use tonic::transport::Channel;

#[cfg(feature = "mode-client")]
use clock_synchronization::grpc_server::clock::sync_clock_client::SyncClockClient;
#[cfg(feature = "mode-client")]
use tonic::transport::Endpoint;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system_clock = Arc::new(AtomicPtr::new(&mut QuartzUtcClock::default()));

    #[cfg(feature = "mode-server")]
    {
        use clock_synchronization::grpc_server::{
            clock::sync_clock_server::SyncClockServer, SyncClockService,
        };
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        use tonic::transport::Server;

        let system_clock = system_clock.clone();
        tokio::spawn(async move {
            let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 50051);
            let sync_clock_service = SyncClockService::with_clock(system_clock);

            println!("Starting Server at {:?}", address.ip());

            Server::builder()
                .add_service(SyncClockServer::new(sync_clock_service))
                .serve(address)
                .await
                .unwrap();
        });
    }

    #[allow(unused_assignments, unused_variables, unused_mut)]
    let mut channel: Option<Channel> = None;

    #[cfg(feature = "mode-client")]
    {
        channel = Some(
            Endpoint::from_static("http://127.0.0.1:50051")
                .connect()
                .await?,
        );
    }

    eframe::run_native(
        "Clocking",
        eframe::NativeOptions {
            initial_window_size: Some(egui::Vec2::new(900.0, 600.0)),
            resizable: false,
            decorated: true,
            ..eframe::NativeOptions::default()
        },
        Box::new(|cc| {
            #[cfg(feature = "mode-client")]
            let client = channel.map(|channel| SyncClockClient::new(channel));

            let app = TemplateApp::new(
                cc,
                system_clock,
                #[cfg(feature = "mode-client")]
                client,
            );
            Box::new(app)
        }),
    );

    Ok(())
}
