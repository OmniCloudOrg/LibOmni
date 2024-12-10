use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord,)]
#[serde(rename_all = "snake_case")]
pub enum CpiCommandType {
    #[serde(rename = "create_vm")]
    CreateVM {
        guest_id: String,
        memory_mb: i32,
        os_type: String,
        resource_pool: String,
        datastore: String,
        vm_name: String,
        cpu_count: i32,
    },
    #[serde(rename = "delete_vm")]
    DeleteVM {
        vm_name: String,
    },
    #[serde(rename = "has_vm")]
    HasVM {
        vm_name: String,
    },
    ConfigureNetworks {
        vm_name: String,
        network_index: i32,
        network_type: String,
    },
    CreateDisk {
        size_mb: i32,
        disk_path: String,
    },
    AttachDisk{
        vm_name: String,
        controller_name: String,
        port: i32,
        disk_path: String,
    },
    DeleteDisk{
        vm_name: String,
        disk_path: String,
    },
    DetachDisk{
        vm_name: String,
        controller_name: String,
        port: i32,
    },
    HasDisk{
        vm_name: String,
        disk_path: String,
    },
    #[serde(rename = "set_vm_metadata")]
    SetVMMetadata{
        vm_name: String,
        key: String,
        value: String,
    },
    CreateSnapshot{
        vm_name: String,
        snapshot_name: String,
    },
    DeleteSnapshot{
        vm_name: String,
        snapshot_name: String,
    },
    HasSnapshot{
        vm_name: String,
        snapshot_name: String,
    },
    GetDisks{
        vm_name: String,
    },
    #[serde(rename = "get_vm")]
    GetVM{
        vm_name: String,
    },
    #[serde(rename = "reboot_vm")]
    RebootVM{
        vm_name: String,
    },
    #[serde(rename = "start_vm")]
    StartVM {
        vm_name: String
    },
    SnapshotDisk{
        disk_path: String,
        snapshot_name: String,
    },
    GetSnapshots{
        vm_name: String,
    },
}

impl ToString for CpiCommandType {
    fn to_string(&self) -> String {
        match self {
            CpiCommandType::CreateVM { .. } => "create_vm".to_string(),
            CpiCommandType::DeleteVM { .. } => "delete_vm".to_string(),
            CpiCommandType::HasVM { .. } => "has_vm".to_string(),
            CpiCommandType::ConfigureNetworks { .. } => "configure_networks".to_string(),
            CpiCommandType::CreateDisk { .. } => "create_disk".to_string(),
            CpiCommandType::AttachDisk { .. } => "attach_disk".to_string(),
            CpiCommandType::DeleteDisk { .. } => "delete_disk".to_string(),
            CpiCommandType::DetachDisk { .. } => "detach_disk".to_string(),
            CpiCommandType::HasDisk { .. } => "has_disk".to_string(),
            CpiCommandType::SetVMMetadata { .. } => "set_vm_metadata".to_string(),
            CpiCommandType::CreateSnapshot { .. } => "create_snapshot".to_string(),
            CpiCommandType::DeleteSnapshot { .. } => "delete_snapshot".to_string(),
            CpiCommandType::HasSnapshot { .. } => "has_snapshot".to_string(),
            CpiCommandType::GetDisks { .. } => "get_disks".to_string(),
            CpiCommandType::GetVM { .. } => "get_vm".to_string(),
            CpiCommandType::RebootVM { .. } => "reboot_vm".to_string(),
            CpiCommandType::SnapshotDisk { .. } => "snapshot_disk".to_string(),
            CpiCommandType::GetSnapshots { .. } => "get_snapshots".to_string(),
            CpiCommandType::StartVM { .. } => "start_vm".to_string(),
        }
    }
}