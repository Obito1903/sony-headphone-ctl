pub mod args;

use args::{Args, Commands};
use clap::Parser;
use xm4::{devices::wf1000xm4::Wf1000xm4, devices::SonyDevice};

async fn process<D: SonyDevice>(args: Args, mut device: D) {
    match args.command {
        Commands::Config(config) => match config {
            args::Config::ANC(ambient_sound) => match ambient_sound {
                args::AmbientSoundControl::Ambient { level, voice } => {
                    device
                        .set_anc(xm4::devices::Anc::AmbientSound { level, voice })
                        .await
                        .unwrap();
                }
                args::AmbientSoundControl::NC { wind } => {
                    device
                        .set_anc(xm4::devices::Anc::NoiseCanceling { wind })
                        .await
                        .unwrap();
                }
                args::AmbientSoundControl::Off => {
                    device.set_anc(xm4::devices::Anc::Off).await.unwrap();
                }
            },
        },
        _ => {
            unimplemented!()
        }
    }
}

#[tokio::main]
async fn main() -> bluer::Result<()> {
    let args = args::Args::parse();

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let devices_addr = adapter.device_addresses().await?;

    for device_addr in devices_addr {
        let bt_device = adapter.device(device_addr).unwrap();
        match bt_device.name().await.unwrap() {
            Some(name) => match name.as_str() {
                "WF-1000XM4" => {
                    process(args, Wf1000xm4::new(device_addr).await.unwrap()).await;
                    break;
                }
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}
