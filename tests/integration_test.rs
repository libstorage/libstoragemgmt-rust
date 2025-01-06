/*
 * Copyright (C) 2017 Red Hat, Inc.
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; If not, see <http://www.gnu.org/licenses/>.
 *
 * Author: Gris Ge <fge@redhat.com>
 */

extern crate lsm;
extern crate rand;

use lsm::{
    local_disk, CachePolicy, Client, Disk, NfsAccess, Pool, RaidType, System, Volume,
    VolumeCreateArgThinP,
};
use rand::Rng;

static SIM_SYS_ID: &'static str = "sim-01";

fn make_connection() -> Client {
    Client::new("simc://", None, None).unwrap()
}

fn random_string(prefix: &str) -> String {
    let rand_str: String = rand::thread_rng().gen_ascii_chars().take(5).collect();
    format!("{}{}", prefix, rand_str)
}

fn random_iqn() -> String {
    random_string("iqn.2017-11.com.example:rust-test-")
}

fn get_sys() -> System {
    let mut c = make_connection();
    let syss = c.systems().unwrap();
    assert_eq!(1, syss.len());
    syss[0].clone()
}

fn get_pool() -> Pool {
    let mut c = make_connection();
    let ps = c.pools().unwrap();
    //TODO(Gris Ge): Find a pool could create volume
    (&ps[1]).clone()
}

fn create_vol(c: &mut Client, pool: &Pool, name: &str) -> Volume {
    c.volume_create(
        pool,
        name,
        lsm::size_human_2_size_bytes("1GiB"),
        &VolumeCreateArgThinP::Default,
    )
    .unwrap()
}

#[test]
fn avail_plugins() {
    let pis = lsm::available_plugins().unwrap();
    for pi in &pis {
        println!("got plugin '{:?}'", pi);
    }
    assert!(!pis.is_empty());
}

#[test]
fn sys() {
    let mut c = make_connection();
    let syss = c.systems().unwrap();
    println!("got systems '{:?}'", syss);
    assert_eq!(1, syss.len());
    assert_eq!(SIM_SYS_ID, syss[0].id);
}

#[test]
fn vol() {
    let mut c = make_connection();
    let pool = get_pool();
    let new_vol = create_vol(&mut c, &pool, &random_string("vol_"));
    println!("new volume '{:?}'", new_vol);
    let new_size = lsm::size_human_2_size_bytes("2GiB");
    let updated_vol = c.volume_resize(&new_vol, new_size).unwrap();
    assert!(updated_vol.size_bytes() >= new_size);
    println!(
        "new size {}({} bytes)",
        lsm::size_bytes_2_size_human(updated_vol.size_bytes()),
        updated_vol.size_bytes()
    );
    let dst_vol = c
        .volume_replicate(
            None,
            lsm::VolumeReplicateType::Clone,
            &updated_vol,
            &random_string("vol_rep_dst_"),
        )
        .unwrap();
    println!("new replicate target volume: '{:?}'", dst_vol);

    println!(
        "voluem replicate range size is {} bytes",
        c.volume_rep_range_blk_size(&get_sys()).unwrap()
    );

    let dst_vol2 = create_vol(&mut c, &pool, &random_string("vol_rep_range_dst_"));
    let ranges = [
        lsm::BlockRange::new(10u64, 50u64, 10u64),
        lsm::BlockRange::new(100u64, 150u64, 10u64),
    ];

    c.volume_replicate_range(
        lsm::VolumeReplicateType::Clone,
        &updated_vol,
        &dst_vol2,
        &ranges,
    )
    .unwrap();

    /* There is no need for us to check whether certain function works or not,
     * that's the job plugin_test. Here we just make sure client binding sent
     * the correct command
     */
    c.volume_disable(&dst_vol).unwrap();
    c.volume_enable(&dst_vol).unwrap();

    c.volume_delete(&dst_vol2).unwrap();
    c.volume_delete(&dst_vol).unwrap();

    let ag = c
        .access_group_create(
            &random_string("ag_"),
            &random_iqn(),
            lsm::InitiatorType::IscsiIqn,
            &get_sys(),
        )
        .unwrap();

    c.volume_mask(&updated_vol, &ag).unwrap();
    c.volume_unmask(&updated_vol, &ag).unwrap();

    let vols = c.volumes().unwrap();
    assert!(vols.len() >= 1);

    c.volume_delete(&updated_vol).unwrap();
}

#[test]
fn pools() {
    let mut c = make_connection();
    let ps = c.pools().unwrap();
    println!("got pools '{:?}'", ps);
    assert_eq!(4, ps.len());
}

#[test]
fn disks() {
    let mut c = make_connection();
    let ds = c.disks().unwrap();
    println!("got disks '{:?}'", ds);
    assert_eq!(18, ds.len());
}

#[test]
fn file_system() {
    let mut c = make_connection();
    let pool = get_pool();
    let size_1gib = lsm::size_human_2_size_bytes("1GiB");
    let fs = c
        .fs_create(&pool, &random_string("fs_"), size_1gib)
        .unwrap();
    let fs = c.fs_resize(&fs, size_1gib * 2).unwrap();
    println!("Got new fs: '{:?}'", fs);
    assert!(fs.total_space >= size_1gib * 2);
    let fss = c.fs().unwrap();
    assert!(fss.len() >= 1);

    let snap = c
        .fs_snapshot_create(&fs, &random_string("fs_snap_"))
        .unwrap();
    println!("Got new fs snapshot: '{:?}'", snap);
    let snaps = c.fs_snapshots(&fs).unwrap();
    assert_eq!(1, snaps.len());

    let dst_fs = c
        .fs_clone(&fs, &random_string("fs_clone_dst_"), Some(&snap))
        .unwrap();
    println!("Got new clone target fs: '{:?}'", dst_fs);

    c.fs_file_clone(&fs, "/root/foo", "/root/foe", Some(&snap))
        .unwrap();

    c.fs_snapshot_restore(&fs, &snap, true, None, None).unwrap();
    c.fs_snapshot_delete(&fs, &snap).unwrap();
    c.fs_delete(&dst_fs).unwrap();
    c.fs_delete(&fs).unwrap();
}

#[test]
fn nfs_export() {
    let mut c = make_connection();
    println!(
        "Supported NFS client authentication types: {:?}",
        c.nfs_exp_auth_type_list().unwrap()
    );
    let pool = get_pool();
    let fs = c
        .fs_create(
            &pool,
            &random_string("fs_"),
            lsm::size_human_2_size_bytes("1GiB"),
        )
        .unwrap();
    let access = NfsAccess {
        root_list: &["localhost"],
        rw_list: &["abc.com", "localhost"],
        ro_list: &["b.com"],
        anon_uid: None,
        anon_gid: None,
    };
    let exp = c
        .fs_export(&fs, Some(&random_string("/")), &access, None, None)
        .unwrap();
    let eps = c.nfs_exports().unwrap();
    assert!(!eps.is_empty());
    println!("got NFS exports '{:?}'", eps);
    c.fs_unexport(&exp).unwrap();
}

#[test]
fn ag() {
    let mut c = make_connection();
    let ag = c
        .access_group_create(
            &random_string("ag_"),
            &random_iqn(),
            lsm::InitiatorType::IscsiIqn,
            &get_sys(),
        )
        .unwrap();
    println!("Created new ag: '{:?}'", ag);

    let ags = c.access_groups().unwrap();
    println!("got access groups '{:?}'", ags);
    assert!(ags.len() >= 1);

    let tmp_init = &random_iqn();
    let ag = c
        .access_group_init_add(&ag, tmp_init, lsm::InitiatorType::IscsiIqn)
        .unwrap();
    println!("updated ag after add init: '{:?}'", ag);
    let ag = c
        .access_group_init_add(&ag, "0x20:00:00:81:23:45:ac:01", lsm::InitiatorType::Wwpn)
        .unwrap();
    let ag = c
        .access_group_init_del(&ag, tmp_init, lsm::InitiatorType::IscsiIqn)
        .unwrap();
    println!("Updated ag after del init: '{:?}'", ag);
    c.access_group_delete(&ag).unwrap();
}

#[test]
fn target_ports() {
    let mut c = make_connection();
    let tps = c.target_ports().unwrap();
    println!("got target ports '{:?}'", tps);
    assert_eq!(5, tps.len());
}

#[test]
fn batteries() {
    let mut c = make_connection();
    let bs = c.batteries().unwrap();
    println!("got batteries '{:?}'", bs);
    assert_eq!(2, bs.len());
}

#[test]
fn tmo() {
    let mut c = make_connection();
    c.time_out_set(10_000).unwrap();
    assert_eq!(10_000, c.time_out_get().unwrap());
    println!("timeout works well");
}

#[test]
fn cap() {
    let mut c = make_connection();
    let cap = c.capabilities(&get_sys()).unwrap();
    println!("got cap '{:?}'", cap);
    assert_eq!(true, cap.is_supported(lsm::Capability::Volumes));
    assert_eq!(true, cap.is_supported(lsm::Capability::DiskVpd83Get));
}

#[test]
fn plugin_info() {
    let mut c = make_connection();
    let pi = c.plugin_info().unwrap();
    println!("got plugin_info '{:?}'", pi);
    assert_eq!("Compiled plug-in example", pi.description);
}

#[test]
fn sys_read_cache_pct() {
    let mut c = make_connection();
    let sys = &get_sys();
    c.sys_read_cache_pct_set(sys, 99).unwrap();
    let sys = &get_sys();
    println!("updated system is: {:?}", sys);
    assert_eq!(99, sys.read_cache_pct);
}

#[test]
fn iscsi_auth() {
    let mut c = make_connection();
    c.iscsi_chap_auth_set(&random_iqn(), None, None, None, None)
        .unwrap();
}

#[test]
fn vol_mask() {
    let mut c = make_connection();
    let ag = c
        .access_group_create(
            &random_string("ag_"),
            &random_iqn(),
            lsm::InitiatorType::IscsiIqn,
            &get_sys(),
        )
        .unwrap();
    let pool = get_pool();
    let vol = create_vol(&mut c, &pool, &random_string("vol_"));
    c.volume_mask(&vol, &ag).unwrap();
    let query_vols = c.vols_masked_to_ag(&ag).unwrap();
    assert_eq!(1, query_vols.len());
    let query_ags = c.ags_granted_to_vol(&vol).unwrap();
    assert_eq!(1, query_ags.len());
    c.volume_unmask(&vol, &ag).unwrap();
    c.volume_delete(&vol).unwrap();
    c.access_group_delete(&ag).unwrap();
}

#[test]
fn vol_child_dep() {
    let mut c = make_connection();
    let pool = get_pool();
    let vol = create_vol(&mut c, &pool, &random_string("vol_"));
    let dst_vol = c
        .volume_replicate(
            None,
            lsm::VolumeReplicateType::Clone,
            &vol,
            &random_string("vol_rep_dst_"),
        )
        .unwrap();
    assert_eq!(true, c.vol_has_child_dep(&vol).unwrap());
    c.vol_child_dep_rm(&vol).unwrap();
    c.volume_delete(&vol).unwrap();
    c.volume_delete(&dst_vol).unwrap();
}

#[test]
fn fs_child_dep() {
    let mut c = make_connection();
    let pool = get_pool();
    let fs = c
        .fs_create(
            &pool,
            &random_string("fs_"),
            lsm::size_human_2_size_bytes("1GiB"),
        )
        .unwrap();
    let dst_fs = c
        .fs_clone(&fs, &random_string("fs_clone_dst_"), None)
        .unwrap();
    assert_eq!(true, c.fs_has_child_dep(&fs, None).unwrap());
    c.fs_child_dep_rm(&fs, None).unwrap();
    c.fs_delete(&fs).unwrap();
    c.fs_delete(&dst_fs).unwrap();
}

#[test]
fn vol_raid_info() {
    let mut c = make_connection();
    let pool = get_pool();
    let vol = create_vol(&mut c, &pool, &random_string("vol_"));
    let vol_raid_info = c.vol_raid_info(&vol).unwrap();
    println!("Volume RAID info: '{:?}'", vol_raid_info);
}

#[test]
fn pool_member_info() {
    let mut c = make_connection();
    let pools = c.pools().unwrap();
    for pool in pools {
        let pmi = c.pool_member_info(&pool).unwrap();
        println!("Pool member info for {}: '{:?}'", pool.id, pmi);
    }
}

#[test]
fn vrc() {
    let sys = get_sys();
    let mut c = make_connection();
    let vrc_cap = c.vol_raid_create_cap_get(&sys).unwrap();
    println!("Volume RAID create capabilities are '{:?}'", vrc_cap);

    let disks = c.disks().unwrap();
    let mut chose_disks: Vec<Disk> = Vec::new();
    for disk in disks {
        if (disk.status & Disk::STATUS_FREE != 0) && chose_disks.len() < 2 {
            chose_disks.push(disk.clone());
        }
    }
    let vol = c
        .vol_raid_create(&random_string("vrc_"), RaidType::Raid1, &chose_disks, None)
        .unwrap();
    println!("Created RAID volume '{:?}'", vol);
    let vol_raid_info = c.vol_raid_info(&vol).unwrap();
    println!("Volume RAID info: '{:?}'", vol_raid_info);

    c.volume_delete(&vol).unwrap();
}

#[test]
fn vci() {
    let mut c = make_connection();
    let pool = get_pool();
    let vol = create_vol(&mut c, &pool, &random_string("vol_"));
    c.vol_phy_disk_cache_set(&vol, CachePolicy::Disabled)
        .unwrap();
    c.vol_write_cache_set(&vol, CachePolicy::Disabled).unwrap();
    c.vol_read_cache_set(&vol, CachePolicy::Disabled).unwrap();
    println!("Voluem cache info: '{:?}'", c.vol_cache_info(&vol).unwrap());
    c.volume_delete(&vol).unwrap();
}

#[test]
fn test_size_human() {
    assert_eq!(lsm::size_human_2_size_bytes("1.9GB"), 1_900_000_000u64);
    assert_eq!(lsm::size_human_2_size_bytes("1KiB"), 1024u64);
    assert_eq!(lsm::size_human_2_size_bytes("1 KiB"), 1024u64);
    assert_eq!(lsm::size_human_2_size_bytes("1 B"), 1u64);
    assert_eq!(lsm::size_human_2_size_bytes("2 K"), 2048u64);
    assert_eq!(lsm::size_human_2_size_bytes("2 k"), 2048u64);
    assert_eq!(lsm::size_human_2_size_bytes("2 KB"), 2000u64);
}

#[test]
fn test_local_disk_list() {
    use lsm::local_disk;
    let disks = local_disk::list().unwrap();

    for d in disks {
        assert!(d.len() > 0);
    }
}

#[test]
fn test_local_disk_vpd() {
    use lsm::local_disk;

    let disks = local_disk::list().unwrap();
    for d in disks {
        if let Ok(vpd) = local_disk::vpd83_get(&d) {
            if let Ok(vpd_search) = local_disk::vpd83_search(&vpd) {
                let mut found = false;

                for i in &vpd_search {
                    if *i == vpd {
                        found = true;
                        break;
                    }
                }
                assert!(
                    found,
                    "{}",
                    format!("device {} with vpd {} not found?", d, vpd)
                );
            }
        }
    }
}

#[test]
fn test_local_disk_serial_num_get() {
    use lsm::local_disk;

    let disks = local_disk::list().unwrap();
    for d in disks {
        if let Ok(sn) = local_disk::serial_num_get(&d) {
            assert!(
                sn.len() > 0,
                "{}",
                format!("Good return for serial_num_get, but value is empty! {}", d)
            );
        }
    }
}

#[test]
fn test_local_disk_health_get() {
    use lsm::local_disk;

    let disks = local_disk::list().unwrap();
    for d in disks {
        if let Ok(_health) = local_disk::health_get(&d) {}
    }
}

#[test]
fn test_local_disk_rpm_get() {
    use lsm::local_disk;

    let disks = local_disk::list().unwrap();
    for d in disks {
        if let Ok(rpm) = local_disk::rpm_get(&d) {
            if d.contains("nvme") {
                assert_eq!(
                    rpm,
                    local_disk::Rpm::NonRotatingMedium,
                    "{}",
                    format!(
                        "Expecting NVMe disk rpm {} to be NonRotatingMedium, not {:?}",
                        d, rpm
                    )
                );
            }
        }
    }
}

#[test]
fn test_local_disk_link_type_get() {
    use lsm::local_disk;

    let disks = local_disk::list().unwrap();
    for d in disks {
        if let Ok(link_type) = local_disk::link_type_get(&d) {
            if d.contains("nvme") {
                assert_eq!(
                    link_type,
                    local_disk::LinkType::Pcie,
                    "{}",
                    format!(
                        "Expecting NVMe disk rpm {} to have Pcie link type, not {:?}",
                        d, link_type
                    )
                );
            }
        }
    }
}

fn led_set(slots: &mut local_disk::LedSlots, slot: &local_disk::LedSlot, new_state: u32) {
    slots.slot_status_set(&slot, new_state).unwrap();
    let current = local_disk::led_status_get(&slot.device().unwrap()).unwrap();
    assert_eq!(new_state, current);

    let state_by_slot = slots.slot_status_get(&slot);
    assert_eq!(new_state, state_by_slot);
}

#[test]
fn test_local_disk_led() {
    use lsm::local_disk;

    let mut slots_handle = local_disk::LedSlots::new().unwrap();

    let slots = slots_handle.slots_get();

    for s in slots.iter() {
        if let Some(device_node) = s.device() {
            let state_by_device_node = local_disk::led_status_get(&device_node).unwrap();
            let state_by_slot = slots_handle.slot_status_get(&s);
            assert_eq!(state_by_device_node, state_by_slot);

            // Turn everything off, then on, then place back to start
            led_set(
                &mut slots_handle,
                &s,
                local_disk::LED_STATUS_IDENT_OFF | local_disk::LED_STATUS_FAULT_OFF,
            );
            led_set(
                &mut slots_handle,
                &s,
                local_disk::LED_STATUS_IDENT_ON | local_disk::LED_STATUS_FAULT_ON,
            );
            led_set(&mut slots_handle, &s, state_by_slot);
        }
    }
}
