#![cfg(feature = "servicediscovery")]

extern crate rusoto_core;
extern crate rusoto_servicediscovery;

use rusoto_core::Region;
use rusoto_servicediscovery::{ListServicesRequest, ServiceDiscovery, ServiceDiscoveryClient};

#[test]
fn should_list_services() {
    let client = ServiceDiscoveryClient::new(Region::UsEast1);
    let request = ListServicesRequest::default();

    let res = client.list_services(request).sync().unwrap();
    println!("Res: {:?}", res);
}
