extern crate libvirt;


// Run with: `cargo test --features=integ-tests`
#[cfg(feature = "integ-tests")]
mod integ_tests {
    use libvirt::connection::*;

    #[test]
    fn it_creates_storage() {
        let conn: Connection = Connection::new("qemu:///session".to_string(), ConnectionType::OPEN)
            .expect("could not connect to qemu");

        // Just in case it exists, delete it
        conn.lookup_storage_pool_byname("integ-test-pool").map(|existing| {
            existing.clone().destroy();
            existing.undefine().unwrap();
        });

        let pool = conn.define_storage_pool(r#"
        <pool type="dir">
            <name>integ-test-pool</name>
            <target>
                <path>/tmp/integ-test-pool</path>
            </target>
        </pool>
        "#)
            .expect("could not create integ-test-pool");

        // OVERWRITE so this test can be rerun more safely
        pool.clone().build(0).unwrap();
        pool.clone().create(0).unwrap();

        let pools = conn.list_storage_pool().unwrap();

        assert!(pools.contains(&("integ-test-pool".to_string())),
                format!("{:?} didn't contain the integ test pool", pools));

        pool.clone().destroy().unwrap();
        pool.undefine().unwrap();

        conn.close().unwrap();
    }
}
