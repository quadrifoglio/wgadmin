//! Handlers for device-related endpoints.

use std::convert;
use std::convert::TryInto;

use actix_web::web;
use actix_web::HttpResponse;

use serde::{Deserialize, Serialize};

use rwg::{Device, Key};

use crate::error::Error;
use crate::peer::PeerDesc;

/// The form of a device creation/update request payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceRequest {
    listen_port: Option<u16>,
    private_key: Option<String>,
    peers: Vec<PeerDesc>,
}

/// The form the payload of response to a request about a device.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceResponse {
    name: String,
    listen_port: Option<u16>,
    public_key: Option<String>,
    peers: Vec<PeerDesc>,
}

impl DeviceRequest {
    /// Create a new device using the parameters from the device request descriptor.
    fn create_device(&self, name: String) -> Result<(), Error> {
        let private_key = match self.private_key.as_ref().map(|key| Key::from_base64(key)) {
            Some(Ok(key)) => Some(key),
            Some(Err(_)) => return Err(Error::InvalidField("device private key")),
            None => None,
        };

        let dev = Device::create(name, private_key).map_err(|err| Error::from(err))?;
        self.apply_to_device(dev)
    }

    /// Apply the parameters from the device request descriptor to the specified device.
    fn apply_to_device(&self, mut dev: Device) -> Result<(), Error> {
        if let Some(port) = self.listen_port {
            dev.set_listen_port(port);
        }

        dev.peers_mut().clear();

        for peer in &self.peers {
            dev.peers_mut().push(peer.clone().try_into()?);
        }

        dev.save().map_err(|err| Error::from(err))
    }
}

impl convert::From<Device> for DeviceResponse {
    fn from(dev: Device) -> Self {
        DeviceResponse {
            name: String::from(dev.name()),
            listen_port: dev.listen_port(),
            public_key: dev.public_key().clone().map(|key| key.to_base64()),
            peers: dev
                .peers()
                .iter()
                .map(|peer| PeerDesc::from(peer))
                .collect::<Vec<_>>(),
        }
    }
}

/// Handler for 'GET /api/devices'.
/// Lists out all WireGuard devices of the machine.
pub async fn list() -> HttpResponse {
    match Device::all() {
        Ok(devices) => HttpResponse::Ok().json(
            devices
                .into_iter()
                .map(|dev| DeviceResponse::from(dev))
                .collect::<Vec<_>>(),
        ),

        Err(err) => Error::from(err).into_http(),
    }
}

/// Handler for 'GET /api/devices/{name}'.
/// Return the informations of the device with the specified name.
pub async fn get(path: web::Path<String>) -> HttpResponse {
    let name = path.into_inner();

    match Device::open(name) {
        Ok(device) => HttpResponse::Ok().json(DeviceResponse::from(device)),
        Err(err) => Error::from(err).into_http(),
    }
}

/// Handler for 'POST /api/devices/{name}'.
/// Create or update a device.
pub async fn post(path: web::Path<String>, desc: web::Json<DeviceRequest>) -> HttpResponse {
    let name = path.into_inner();
    let desc = desc.into_inner();

    match Device::open(&name) {
        Ok(dev) => match desc.apply_to_device(dev) {
            Ok(_) => HttpResponse::Ok().json(desc),
            Err(err) => Error::from(err).into_http(),
        },

        Err(err) => match Error::from(err) {
            Error::NotFound => match desc.create_device(name) {
                Ok(_) => HttpResponse::Ok().json(desc),
                Err(err) => err.into_http(),
            },

            other => other.into_http(),
        },
    }
}
