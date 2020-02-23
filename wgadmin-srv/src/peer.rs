//! WireGuard peer management.

use std::convert;

use serde::{Deserialize, Serialize};

use rwg::{AllowedIp, Endpoint, Key, Peer};

use crate::error::Error;

/// Description of an allowed IP of a peer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowedIpDesc {
    address: String,
    netmask: u8,
}

impl convert::From<&AllowedIp> for AllowedIpDesc {
    fn from(ip: &AllowedIp) -> Self {
        AllowedIpDesc {
            address: format!("{}", ip.addr()),
            netmask: ip.mask(),
        }
    }
}

impl convert::TryInto<AllowedIp> for AllowedIpDesc {
    type Error = Error;

    fn try_into(self) -> Result<AllowedIp, Self::Error> {
        let addr = self
            .address
            .parse()
            .map_err(|_| Error::InvalidField("allowed ip address"))?;

        Ok(AllowedIp::new(addr, self.netmask))
    }
}

/// Description of the internet endpoint of a peer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointDesc {
    address: String,
    port: u16,
}

impl convert::TryInto<Endpoint> for EndpointDesc {
    type Error = Error;

    fn try_into(self) -> Result<Endpoint, Self::Error> {
        let addr = self
            .address
            .parse()
            .map_err(|_| Error::InvalidField("endpoint address"))?;

        Ok((addr, self.port))
    }
}

/// Description of a peer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeerDesc {
    public_key: Option<String>,
    endpoint: Option<EndpointDesc>,
    allowed_ips: Vec<AllowedIpDesc>,
}

impl convert::From<&Peer> for PeerDesc {
    fn from(peer: &Peer) -> Self {
        PeerDesc {
            public_key: peer.public_key().map(|key| key.to_base64()),
            endpoint: peer.endpoint().map(|(addr, port)| EndpointDesc {
                address: format!("{}", addr),
                port: *port,
            }),
            allowed_ips: peer
                .allowed_ips()
                .iter()
                .map(|ip| AllowedIpDesc::from(ip))
                .collect::<Vec<_>>(),
        }
    }
}

impl convert::TryInto<Peer> for PeerDesc {
    type Error = Error;

    fn try_into(self) -> Result<Peer, Self::Error> {
        let public_key = match self.public_key {
            Some(ref public_key) => {
                Key::from_base64(public_key).map_err(|_| Error::InvalidField("peer public key"))?
            }

            None => return Err(Error::MissingField("peer public key")),
        };

        let endpoint = match self.endpoint {
            Some(endpoint) => Some(endpoint.try_into()?),
            None => None,
        };

        let mut peer = Peer::new(public_key, endpoint);

        for allowed_ip in self.allowed_ips {
            peer.add_allowed_ip(allowed_ip.try_into()?);
        }

        Ok(peer)
    }
}
