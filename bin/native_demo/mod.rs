mod app;
pub mod state;

use app::TemplateApp;
use clock_synchronization::quartz_clock::QuartzUtcClock;
use eframe::egui;
use std::sync::{atomic::AtomicPtr, Arc};

#[cfg(feature = "slave")]
use {
    clock_synchronization::grpc_server::clock::sync_clock_client::SyncClockClient,
    tonic::transport::{Channel, Endpoint},
};

#[cfg(feature = "dhat-profile")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(all(feature = "master", feature = "slave"))]
    compile_error!("A process cannot be Master and Slave at the same time");

    #[cfg(feature = "dhat-profile")]
    let _profiler = dhat::Profiler::new_heap();

    let system_clock = Arc::new(AtomicPtr::new(&mut QuartzUtcClock::default()));

    #[cfg(feature = "master")]
    start_master_server(system_clock.clone());

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
                connect_slave_to_server(),
            );
            Box::new(app)
        }),
    );

    Ok(())
}

#[cfg(feature = "master")]
fn start_master_server(system_clock: Arc<AtomicPtr<QuartzUtcClock>>) {
    use clock_synchronization::grpc_server::{
        clock::sync_clock_server::SyncClockServer, SyncClockService,
    };
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use tonic::transport::Server;

    tokio::spawn(async move {
        let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 50051);
        let sync_clock_service = SyncClockService::with_clock(system_clock);

        println!("Starting Master at {:?}", address);

        Server::builder()
            .add_service(SyncClockServer::new(sync_clock_service))
            .serve(address)
            .await
            .unwrap();
    });
}

#[cfg(feature = "slave")]
fn connect_slave_to_server() -> SyncClockClient<Channel> {
    println!("Slave Connecting to Master");
    SyncClockClient::new(Endpoint::from_static("http://127.0.0.1:50051").connect_lazy())
}
