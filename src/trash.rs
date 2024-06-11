#[link(name = "shell32")]
extern "system" {
    pub fn SHFileOperationW(lpFileOp: *mut SHFILEOPSTRUCTW) -> i32;
}

pub const FO_DELETE: u16 = 0x0003;
pub const FOF_ALLOWUNDO: u16 = 0x0040;
pub const FOF_WANTNUKEWARNING: u16 = 0x4000;

#[derive(Debug)]
#[repr(C)]
pub struct SHFILEOPSTRUCTW {
    pub hwnd: *mut std::ffi::c_void,
    /// A value that indicates which operation to perform.
    pub w_func: u32,
    /// A pointer to one or more source file names.
    /// This string must be double-null terminated.
    pub p_from: *const u16,
    /// A pointer to the destination file or directory name.
    /// This parameter must be set to NULL if it is not used.
    /// This string must be double-null terminated.
    pub p_to: *const u16,
    ///Flags that control the file operation.
    pub f_flags: u16,
    /// When the function returns, this member contains TRUE if any file operations were aborted before they were completed; otherwise, FALSE.
    /// An operation can be manually aborted by the user through UI or it can be silently aborted by the system if the FOF_NOERRORUI or FOF_NOCONFIRMATION flags were set.
    pub f_any_operations_aborted: i32,
    pub h_name_mappings: *mut std::ffi::c_void,
    /// A pointer to the title of a progress dialog box.
    /// This is a null-terminated string.
    /// This member is used only if fFlags includes the FOF_SIMPLEPROGRESS flag.
    pub l_psz_progress_title: *const u16,
}

#[must_use]
/// Send a file/folder to the Recycle Bin.
/// You should use fully qualified path names with this function.
pub fn trash<P: AsRef<std::path::Path>>(path: P) -> Result<(), &'static str> {
    use std::os::windows::ffi::OsStrExt;

    let path = path.as_ref();

    if !path.is_absolute() {
        return Err("Path must be absolute.");
    }

    //This string must be double-null terminated.
    let os_path: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .chain(Some(0))
        .collect();

    let mut fileop = SHFILEOPSTRUCTW {
        hwnd: std::ptr::null_mut(),
        w_func: FO_DELETE as u32,
        p_from: os_path.as_ptr(),
        p_to: std::ptr::null(),
        f_flags: FOF_ALLOWUNDO | FOF_WANTNUKEWARNING,
        f_any_operations_aborted: 0,
        h_name_mappings: std::ptr::null_mut(),
        l_psz_progress_title: std::ptr::null(),
    };

    unsafe {
        let result = SHFileOperationW(&mut fileop);
        match result {
            0x0 => {Ok(())}
            0x2 => Err("The system cannot find the file specified."),
            0x5 => Err("Access is denied."),
            0x71 => Err("The source and destination files are the same file."),
            0x72 => Err("Multiple file paths were specified in the source buffer, but only one destination file path."),
            0x73 => Err("Rename operation was specified but the destination path is a different directory. Use the move operation instead."),
            0x74 => Err("The source is a root directory, which cannot be moved or renamed."),
            0x75 => Err("The operation was canceled by the user, or silently canceled if the appropriate flags were supplied to SHFileOperation."),
            0x76 => Err("The destination is a subtree of the source."),
            0x78 => Err("Security settings denied access to the source."),
            0x79 => Err("The source or destination path exceeded or would exceed MAX_PATH."),
            0x7A => Err("The operation involved multiple destination paths, which can fail in the case of a move operation."),
            0x7C => Err("The path in the source or destination or both was invalid."),
            0x7D => Err("The source and destination have the same parent folder."),
            0x7E => Err("The destination path is an existing file."),
            0x80 => Err("The destination path is an existing folder."),
            0x81 => Err("The name of the file exceeds MAX_PATH."),
            0x82 => Err("The destination is a read-only CD-ROM, possibly unformatted."),
            0x83 => Err("The destination is a read-only DVD, possibly unformatted."),
            0x84 => Err("The destination is a writable CD-ROM, possibly unformatted."),
            0x85 => Err("The file involved in the operation is too large for the destination media or file system."),
            0x86 => Err("The source is a read-only CD-ROM, possibly unformatted."),
            0x87 => Err("The source is a read-only DVD, possibly unformatted."),
            0x88 => Err("The source is a writable CD-ROM, possibly unformatted."),
            0xB7 => Err("MAX_PATH was exceeded during the operation."),
            0x10000 => Err("An unspecified error occurred on the destination."),
            0x10074 => Err("Destination is a root directory and cannot be renamed."),
            _ => panic!("Unknown error: {:#02x}", result),
        }
    }
}
