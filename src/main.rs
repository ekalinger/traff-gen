use netmap_rs::prelude::*;

fn main() -> Result<(), Error> {
    // Open the netmap interface
    let nm = NetmapBuilder::new("ens224np2")
        .num_tx_rings(1)
        .num_rx_rings(1)
        .build()?;

    let mut tx_ring = nm.tx_ring(0)?;

    // Send a packet (zero-copy)
    tx_ring.send(b"hello netmap!")?;
    tx_ring.sync(); // Force hardware transmission

    Ok(())
}
