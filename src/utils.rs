use std::slice;
use std::str;

pub(crate) fn cstr_to_str(c_str: *const i8) -> Option<&'static str> {
    if c_str.is_null() {
        return None; // 检查空指针
    }

    unsafe {
        // 获取长度（假设是 C 风格字符串）
        let len = libc::strlen(c_str);
        // 转换为 *const u8
        let bytes = slice::from_raw_parts(c_str as *const u8, len);
        // 转换为 &str（假设输入是 UTF-8 编码）
        str::from_utf8(bytes).ok()
    }
}
