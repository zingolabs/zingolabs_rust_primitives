//! Tower service type definitions shared between crates

use http_body_util::combinators::UnsyncBoxBody;
use tonic::Status;
use tower::util::BoxCloneService;
use zcash_primitives::consensus::BlockHeight;
use zcash_protocol::consensus::NetworkUpgrade;

/// The underlying service type used for gRPC connections
pub type UnderlyingTowerService = BoxCloneService<
    http::Request<UnsyncBoxBody<prost::bytes::Bytes, Status>>,
    http::Response<hyper::body::Incoming>,
    hyper_util::client::legacy::Error,
>;

/// Network types
#[derive(Clone, Copy)]
pub enum Network {
    /// Regtest
    Regtest,
    /// Testnet
    Testnet,
    /// Mainnet
    Mainnet,
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mainnet => write!(f, "Mainnet"),
            Self::Testnet => write!(f, "Testnet"),
            Self::Regtest => write!(f, "Regtest"),
        }
    }
}
/// Activation heights for local network upgrades
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActivationHeights {
    /// Overwinter network upgrade activation height
    pub overwinter: BlockHeight,
    /// Sapling network upgrade activation height
    pub sapling: BlockHeight,
    /// Blossom network upgrade activation height
    pub blossom: BlockHeight,
    /// Heartwood network upgrade activation height
    pub heartwood: BlockHeight,
    /// Canopy network upgrade activation height
    pub canopy: BlockHeight,
    /// Nu5 (a.k.a. Orchard) network upgrade activation height
    pub nu5: BlockHeight,
    /// Nu6 network upgrade activation height
    pub nu6: BlockHeight,
}

impl Default for ActivationHeights {
    fn default() -> Self {
        Self {
            overwinter: 1.into(),
            sapling: 1.into(),
            blossom: 1.into(),
            heartwood: 1.into(),
            canopy: 1.into(),
            nu5: 1.into(),
            nu6: 1.into(),
        }
    }
}

impl ActivationHeights {
    /// Returns activation height for given `network_upgrade`.
    pub fn activation_height(&self, network_upgrade: NetworkUpgrade) -> BlockHeight {
        match network_upgrade {
            NetworkUpgrade::Overwinter => self.overwinter,
            NetworkUpgrade::Sapling => self.sapling,
            NetworkUpgrade::Blossom => self.blossom,
            NetworkUpgrade::Heartwood => self.heartwood,
            NetworkUpgrade::Canopy => self.canopy,
            NetworkUpgrade::Nu5 => self.nu5,
            NetworkUpgrade::Nu6 => self.nu6,
        }
    }
}
pub(crate) const LOCALHOST_IPV4: &str = "http://127.0.0.1";
/// Constructs a URI with the localhost IPv4 address and the specified port.
pub fn localhost_uri(port: portpicker::Port) -> http::Uri {
    format!("{}:{}", LOCALHOST_IPV4, port).try_into().unwrap()
}
