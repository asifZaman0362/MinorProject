use std::{collections::HashMap, io::Bytes};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct IpAddress {
    ip_address: u32
}

pub struct TransferServer {
    
    ip_address: IpAddress,
    domain: String,
    lookup_table: HashMap<String, IpAddress>

}

impl TransferServer {

    pub fn new(ip_address: IpAddress, domain: String) -> TransferServer {
        let lookup_table = HashMap::<String, IpAddress>::new();
        TransferServer {
            ip_address,
            domain,
            lookup_table
        }
    }

    pub async fn find_ip_address(&self, username: String, domain: String) -> Result<IpAddress, &'static str> {
        if domain == self.domain {
            if self.lookup_table.contains_key(&username) {
                Ok(self.lookup_table[&username])
            } else {
                Err("Address of reciever could not be resolved!")
            }
        } else {
            Err("Address of reciever could not be resolved!")
        }
    }

    pub async fn transfer_mail(&self, mail: Vec<u8>, destination: String) -> u8 {
        panic!("Not implemented!")
    }

}
