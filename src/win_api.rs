use std::io::{Error};

use crate::bindings::{
    Windows::Win32::Foundation::PWSTR,
    Windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW,
    Windows::Win32::Storage::FileSystem::GetDriveTypeW,
    Windows::Win32::Storage::FileSystem::GetLogicalDrives,
    Windows::Win32::Storage::FileSystem::GetVolumeInformationW,
};

/// Creates Rust String from vector u16
fn vec_u16_to_string(vec: &Vec<u16>) -> String {
    let mut index = 0;
    for item in 0..vec.len() {
        if vec[item] == 0 {
            break;
        }
        index = item + 1;
    }
    String::from_utf16_lossy(&vec[0..index])
}

/// Defines different drive types according to [GetDriveTypeW](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getdrivetypew)
#[derive(Debug)]
pub enum DriveType {
    /// The drive type cannot be determined
    DriveUnknown = 0,
    /// The root path is invalid; for example, there is no volume mounted at the specified path
    DriveNoRootDir = 1,
    /// The drive has removable media; for example, a floppy drive, thumb drive, or flash card reader
    DriveRemovable = 2,
    /// The drive has fixed media; for example, a hard disk drive or flash drive.
    DriveFixed = 3,
    /// The drive is a remote (network) drive.
    DriveRemote = 4,
    /// The drive is a CD-ROM drive.
    DriveCDRom = 5,
    /// The drive is a RAM disk.
    DriveRamDisk = 6,
}

impl From<u32> for DriveType {
    fn from(index: u32) -> Self {
        match index {
            0 => DriveType::DriveUnknown,
            1 => DriveType::DriveNoRootDir,
            2 => DriveType::DriveRemovable,
            3 => DriveType::DriveFixed,
            4 => DriveType::DriveRemote,
            5 => DriveType::DriveCDRom,
            6 => DriveType::DriveRamDisk,
            _ => {
                panic!("Invalid Drive Type")
            }
        }
    }
}

/// Use [GetVolumeInformationW](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getvolumeinformationw) API function
/// and returns tuple of (volume name, file system name,volume serial, max length, file system flags)
///
/// Minimum OS Version: Windows XP/Windows Server 2003
pub fn get_volume_information(
    lprootpathname: String
) -> Result<(String, String, u32, u32, u32), Error> {
    // Maximum Volume name length is 32 characters which is equivalent to 64 unicode bytes
    let mut volume_name_buf: Vec<u16> = Vec::with_capacity(64);
    volume_name_buf.resize(64, 0);

    let mut file_system_name_buf: Vec<u16> = Vec::with_capacity(255);
    file_system_name_buf.resize(255, 0);

    let pwstr_volume_name: PWSTR = PWSTR(volume_name_buf.as_mut_ptr());
    let pwstr_file_system_name: PWSTR = PWSTR(file_system_name_buf.as_mut_ptr());

    let mut lpvolumeserialnumber: u32 = 0;
    let mut lpmaximumcomponentlength: u32 = 0;
    let mut lpfilesystemflags: u32 = 0;
    let result = unsafe {
        GetVolumeInformationW(
            lprootpathname,
            pwstr_volume_name,
            volume_name_buf.capacity() as u32,
            &mut lpvolumeserialnumber,
            &mut lpmaximumcomponentlength,
            &mut lpfilesystemflags,
            pwstr_file_system_name,
            file_system_name_buf.capacity() as u32).as_bool()
    };

    if result {
        let result_volume_name = vec_u16_to_string(&volume_name_buf);
        let result_volume_system_name = vec_u16_to_string(&file_system_name_buf);
        Ok((result_volume_name, result_volume_system_name, lpvolumeserialnumber, lpmaximumcomponentlength, lpfilesystemflags))
    } else {
        Err(Error::last_os_error())
    }
}

/// Get drive type by calling [GetDriveTypeW](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getdrivetypew)
/// API function.
///
/// Minimum OS: Windows XP/Windows Server 2003
pub fn get_drive_type(
    lprootpathname: String,
) -> DriveType {
    let result = unsafe {
        GetDriveTypeW(
            lprootpathname
        )
    };

    DriveType::from(result)
}

/// Calls [GetDiskFreeSpaceW](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getdiskfreespacew)
/// Windows API and returns tuple of (free bytes available to caller, total number of bytes, total number of free bytes)
///
/// Minimum OS: Windows XP/Windows Server 2003
pub fn get_disk_free_space(
    lpdirectoryname: String
) -> Result<(u64, u64, u64), Error> {
    let mut lpfreebytesavailabletocaller: u64 = 0;
    let mut lptotalnumberofbytes: u64 = 0;
    let mut lptotalnumberoffreebytes: u64 = 0;
    let result =
        unsafe {
            GetDiskFreeSpaceExW(
                lpdirectoryname,
                &mut lpfreebytesavailabletocaller,
                &mut lptotalnumberofbytes,
                &mut lptotalnumberoffreebytes).as_bool()
        };

    if result {
        Ok((lpfreebytesavailabletocaller, lptotalnumberofbytes, lptotalnumberoffreebytes))
    } else {
        Err(Error::last_os_error())
    }
}

/// Calls [GetLogicalDrives](https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getlogicaldrives) Windows API function
/// and returns Vector of drive letters
pub fn get_logical_drive() -> Result<Vec<char>, Error> {
    let bitmask = unsafe { GetLogicalDrives() };
    if bitmask == 0 {
        Err(Error::last_os_error())
    } else {
        let mut mask = 1;
        let mut result: Vec<char> = vec![];

        for index in 1..=26 {
            if mask & bitmask == mask {
                let char = std::char::from_u32(index + 64);
                result.push(char.unwrap());
            }
            mask = mask << 1;
        }

        Ok(result)
    }
}
