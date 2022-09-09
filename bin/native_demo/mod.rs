mod app;
pub mod state;

use std::sync::{atomic::AtomicPtr, Arc};

use app::TemplateApp;
use clock_synchronization::quartz_clock::QuartzUtcClock;
use eframe::egui;

#[cfg(feature = "slave")]
use clock_synchronization::grpc_server::clock::sync_clock_client::SyncClockClient;
#[cfg(feature = "slave")]
use tonic::transport::Endpoint;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(all(feature = "master", feature = "slave"))]
    compile_error!("A process cannot be Master and Slave at the same time");

    #[cfg(feature = "puffin-profile")]
    start_puffin_server();

    let system_clock = Arc::new(AtomicPtr::new(&mut QuartzUtcClock::default()));

    #[cfg(feature = "master")]
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

            println!("Starting Master at {:?}", address.ip());

            Server::builder()
                .add_service(SyncClockServer::new(sync_clock_service))
                .serve(address)
                .await
                .unwrap();
        });
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
            let app = TemplateApp::new(
                cc,
                system_clock,
                #[cfg(feature = "slave")]
                {
                    println!("Slave Connecting to Master");
                    SyncClockClient::new(
                        Endpoint::from_static("http://127.0.0.1:50051").connect_lazy(),
                    )
                },
            );
            Box::new(app)
        }),
    );

    Ok(())
}

#[cfg(feature = "puffin-profile")]
fn start_puffin_server() {
    puffin::set_scopes_on(true);

    match puffin_http::Server::new("0.0.0.0:8585") {
        Ok(puffin_server) => {
            eprintln!("Run:  cargo install puffin_viewer && puffin_viewer --url 127.0.0.1:8585");
            std::mem::forget(puffin_server);
        }
        Err(err) => eprintln!("Failed to start puffin server: {}", err),
    };
}
