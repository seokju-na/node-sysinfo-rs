#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use sysinfo::Networks;

#[napi(object)]
pub struct NetworkInterface {
  pub name: String,
  pub ip_addresses: Vec<String>,
  pub mac_address: String,
}

pub struct NetworkInterfacesTask {
  pub(crate) networks: Networks,
}

#[napi]
impl Task for NetworkInterfacesTask {
  type Output = Vec<NetworkInterface>;
  type JsValue = Vec<NetworkInterface>;

  fn compute(&mut self) -> Result<Self::Output> {
    self.networks.refresh(true);
    let mut mac_addresses = Vec::with_capacity(self.networks.len());
    for (interface, network) in &self.networks {
      mac_addresses.push(NetworkInterface {
        name: interface.to_string(),
        ip_addresses: network
          .ip_networks()
          .iter()
          .map(|x| x.to_string())
          .collect(),
        mac_address: network.mac_address().to_string(),
      });
    }
    Ok(mac_addresses)
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}

#[napi]
pub fn get_network_interfaces() -> AsyncTask<NetworkInterfacesTask> {
  AsyncTask::new(NetworkInterfacesTask {
    networks: Networks::new(),
  })
}
