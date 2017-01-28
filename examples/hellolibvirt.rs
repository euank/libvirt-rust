extern crate libvirt;
extern crate libvirt_sys as virt;

use libvirt::connection::*;

fn hypervisor_info(conn: Connection) {

    let hv_type = match conn.connection_type() {
        Ok(t) => t,
        Err(e) => panic!("Failed to get hypervisor type: {}", e.message),
    };

    let mut hv_ver = match conn.version() {
        Ok(v) => v,
        Err(e) => panic!("Failed to get hypervisor version: {}", e.message),
    };

    let major = hv_ver / 1000000;
    hv_ver %= 1000000;
    let minor = hv_ver / 1000;
    let release = hv_ver % 1000;

    println!("Hypervisor: '{}' version: {}.{}.{}\n",
             hv_type,
             major,
             minor,
             release);

}

fn show_domains(conn: Connection) {

    let active = match conn.count_domain() {
        Ok(d) => d,
        Err(e) => panic!("Failed to get number of active domains: {}", e.message),
    };

    let inactive = match conn.count_defined_domain() {
        Ok(d) => d,
        Err(e) => panic!("Failed to get number of inactive domains: {}", e.message),
    };

    println!("There are {} active and {} inactive domains",
             active,
             inactive);


    let domains = match conn.list_all_domains(virt::VIR_CONNECT_LIST_DOMAINS_ACTIVE |
                                              virt::VIR_CONNECT_LIST_DOMAINS_INACTIVE) {
        Ok(d) => d,
        Err(e) => panic!("Failed to get a list of all domains: {}", e.message),
    };

    for domain in domains {

        let active = match domain.clone().active() {
            Ok(_) => true,
            Err(_) => false,
        };

        let domname = match domain.clone().name() {
            Ok(n) => n,
            Err(e) => panic!("Cannot get name of domain: {}", e.message),
        };

        println!("{} ({})",
                 domname,
                 match active {
                     true => "active",
                     false => "non-active",
                 });

        match domain.free() {
            Ok(_) => {}
            Err(e) => panic!("Cannot free domain: {}", e.message),
        }

    }
}

fn main() {
    let conn: Connection = match Connection::new("qemu:///system".to_string(),
                                                 ConnectionType::READONLY) {
        Ok(c) => c,
        Err(e) => panic!("Failed to connect to hypervisor: {}", e.message),
    };

    println!("Connected to hypervisor");

    hypervisor_info(conn.clone());

    show_domains(conn.clone());

    match conn.close() {
        Ok(()) => println!("Disconnected from hypervisor"),
        Err(e) => panic!("Failed to disconnect from hypervisor: {}", e.message),
    }

}
