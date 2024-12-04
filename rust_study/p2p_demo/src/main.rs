use libp2p::ping::{Ping, PingConfig};
use std::error::Error;
use libp2p::{identity, mdns, Multiaddr, PeerId};
use libp2p::futures::StreamExt;
use libp2p::mdns::{Mdns, MdnsConfig, MdnsEvent};
use libp2p::swarm::{Swarm, SwarmEvent};


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let new_key = identity::Keypair::generate_ed25519();
    let new_peer_id = PeerId::from(new_key.public());
    println!("New Perr Id is {:?}", new_peer_id);

    let transport = libp2p::development_transport(new_key).await?;
    // let behaviour = DummyBehaviour::default();//定义假的网络行为
    // let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));//ping
    let behaviour = Mdns::new(MdnsConfig::default()).await?;//发现peer，在libp2p中实现mDNS将自动发现本地网络上其他libp2p节点

    let mut swarm = Swarm::new(transport, behaviour, new_peer_id);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;


    if let Some(remote_peer) = std::env::args().nth(1) {
        let remote_peer_multiaddr: Multiaddr = remote_peer.parse()?;
        swarm.dial(remote_peer_multiaddr)?;
        println!("Dial remote peer: {:?}", remote_peer);
    }
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on Local Address {:?}", address);
            }
            SwarmEvent::Behaviour(MdnsEvent::Discovered(peers)) =>
                {
                    for (peer, addr) in peers {
                        println!("discovered {} {}",peer,addr);
                    }
                }
            SwarmEvent::Behaviour(
                MdnsEvent::Expired(expired))=>{
                for (peer, addr) in expired {
                    println!("expired {} {}",peer,addr);
                }
            }
            _ => {}
        }
    }
}
