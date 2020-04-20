# Rust binding of LibStorageMgmt

`LibStorageMgmt` provides a set of API for programmatically manage their
storage hardware in a vendor neutral way supporting these actions:

 * List storage pools, volumes, access groups, or file systems.

 * Create and delete volumes, access groups, file systems, or NFS exports.

 * Grant and remove access to volumes, access groups, or initiators.

 * Replicate volumes with snapshots, clones, and copies.

 * Create and delete access groups and edit members of a group.

 * List Linux local SCSI/ATA/NVMe disks.

 * Control IDENT/FAULT LED of local disk via SES(SCSI Enclosure Service).

To use `LibStorageMgmt` rust binding, you need:

 * Start the libstoragemgmt daemon(`lsmd`)

 * Choose a URI after reading [`LibStorageMgmt` user guide][1]

 * Make a connection to plugin via [`lsm::Client`][2].

 * Check required [`lsm::Capabilities`][3] for supported functionality.

 * Invoke required method of [`lsm::Client`][2].

 * Follow [the crate document][4]

# Example code using simulator plugin

```rust
extern crate lsm;
use lsm::{Client, LsmError};
fn main() {
    let mut c: Client = match Client::new("sim://", None, None) {
        Ok(i) => i,
        Err(e) => {
            match e {
                // Error handling goes here
                LsmError::DaemonNotRunning(_) =>
                    panic!("Please start the libstoragemgmt daemon"),
                _ => panic!(e)
            };
        },
    };
    let syss = match c.systems() {
        Ok(i) => i,
        Err(e) => panic!(e)         // Please use error handling as above.
    };
    for s in syss {
        let cap = match c.capabilities(&s) {
            Ok(i) => i,
            Err(e) => panic!(e)     // Please use error handling as above.
        };
        if cap.is_supported(lsm::Capability::Volumes) {
            let vols = match c.volumes() {
                Ok(i) => i,
                Err(e) => panic!(e) // Please use error handling as above.
            };
            for vol in vols {
                println!("Got volume: {} {}", vol.name, vol.id);
            }
        }
    }
}
```

[1]: https://libstorage.github.io/libstoragemgmt-doc/doc/user_guide.html
[2]: https://libstorage.github.io/libstoragemgmt-rust/lsm/struct.Client.html
[3]: https://libstorage.github.io/libstoragemgmt-rust/lsm/struct.Capabilities.html
[4]: https://libstorage.github.io/libstoragemgmt-rust/
