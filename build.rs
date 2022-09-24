fn main() {
    windows::build! {
      Windows::Win32::Storage::FileSystem::GetLogicalDrives,
      Windows::Win32::Storage::FileSystem::GetVolumeInformationW,
      Windows::Win32::Storage::FileSystem::GetDriveTypeW,
      Windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW
    };
}