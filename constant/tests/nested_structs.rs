use constant::Constdef;

#[derive(Constdef)]
pub struct SystemLoad {
    disk: DiskLoad,
    net: NetLoad,
}

#[derive(Constdef)]
pub struct DiskLoad {
    disk_count: usize,
    media_list: Vec<String>,
}

#[derive(Constdef)]
pub struct NetLoad {
    interface_list: Vec<String>,
    portload: [usize; 65536],
}

static mut SYSLOAD: SystemLoad = SystemLoad::default();

#[test]
fn test_system_load_nested_struct() {
    unsafe {
        // check the number of disks
        assert_eq!(SYSLOAD.disk.disk_count, 0);
        // increment the number of disks
        SYSLOAD.disk.disk_count += 1;
        assert_eq!(SYSLOAD.disk.disk_count, 1);
        // check port 80 load
        assert_eq!(SYSLOAD.net.portload[80], 0);
        // increment the load
        SYSLOAD.net.portload[80] += 1;
        assert_eq!(SYSLOAD.net.portload[80], 1);

        // now let's add a disk
        SYSLOAD.disk.media_list.push("/dev/sda1".to_string());
        // now let's add a network interface
        SYSLOAD.net.interface_list.push("eth01".to_string());
    }
}
