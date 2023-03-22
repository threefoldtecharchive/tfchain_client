use tfchain_client::client::RuntimeClient;
use tfchain_client::dynamic;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dyn_cl = dynamic::DynamicClient::new("wss://tfchain.grid.tf:443").await?;

    // let block_before_upgrade = dyn_cl.hash_at_height(Some(5710579 as u32)).await?;
    // let time = dyn_cl.timestamp(block_before_upgrade).await?;
    // print!("time: {}", time);
    // let node = dyn_cl.node(1, block_before_upgrade).await?;
    // if let Some(node) = node {
    //     println!("node before upgrade found: {:?}", node);
    // }

    // let block_after_upgrade = dyn_cl.hash_at_height(Some(6108046 as u32)).await?;
    // let node = dyn_cl.node(9, block_after_upgrade).await?;
    // if let Some(node) = node {
    //     println!("node after upgrade found: {:?}", node);
    // }

    let bl = dyn_cl.hash_at_height(Some(5710579 as u32)).await?;
    let events = dyn_cl.events(bl).await?;
    for e in events.iter() {
        println!("events: {:?}", e);
    }

    Ok(())
}
