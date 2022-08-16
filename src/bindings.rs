/* automatically generated by rust-bindgen 0.60.1 */

pub const LIRC_INET_PORT: u32 = 8765;
pub const LIRC_DRIVER_DEVICE: &[u8; 12usize] = b"/dev/lirc/0\0";
pub const LIRCDOLDCFGFILE: &[u8; 16usize] = b"/etc/lircd.conf\0";
pub const LIRCMDOLDCFGFILE: &[u8; 17usize] = b"/etc/lircmd.conf\0";
pub const LIRCRC_USER_FILE: &[u8; 8usize] = b".lircrc\0";
pub const LIRCRC_OLD_ROOT_FILE: &[u8; 12usize] = b"/etc/lircrc\0";
pub const LIRC_RELEASE_SUFFIX: &[u8; 6usize] = b"_EVUP\0";
pub const LIRC_OPTIONS_PATH: &[u8; 28usize] = b"/etc/lirc/lirc_options.conf\0";
pub const LIRC_OPTIONS_VAR: &[u8; 18usize] = b"LIRC_OPTIONS_PATH\0";
pub const LIRC_EOF: u32 = 134217728;
pub const LIRC_RET_SUCCESS: u32 = 0;
pub const LIRC_RET_ERROR: i32 = -1;
pub type size_t = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lirc_list {
    pub string: *mut ::std::os::raw::c_char,
    pub next: *mut lirc_list,
}

#[test]
fn bindgen_test_layout_lirc_list() {
    assert_eq!(
        ::std::mem::size_of::<lirc_list>(),
        8usize,
        concat!("Size of: ", stringify!(lirc_list))
    );
    assert_eq!(
        ::std::mem::align_of::<lirc_list>(),
        4usize,
        concat!("Alignment of ", stringify!(lirc_list))
    );
    fn test_field_string() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_list>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).string) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_list),
                "::",
                stringify!(string)
            )
        );
    }
    test_field_string();
    fn test_field_next() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_list>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_list),
                "::",
                stringify!(next)
            )
        );
    }
    test_field_next();
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lirc_code {
    pub remote: *mut ::std::os::raw::c_char,
    pub button: *mut ::std::os::raw::c_char,
    pub next: *mut lirc_code,
}

#[test]
fn bindgen_test_layout_lirc_code() {
    assert_eq!(
        ::std::mem::size_of::<lirc_code>(),
        12usize,
        concat!("Size of: ", stringify!(lirc_code))
    );
    assert_eq!(
        ::std::mem::align_of::<lirc_code>(),
        4usize,
        concat!("Alignment of ", stringify!(lirc_code))
    );
    fn test_field_remote() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_code>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).remote) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_code),
                "::",
                stringify!(remote)
            )
        );
    }
    test_field_remote();
    fn test_field_button() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_code>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).button) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_code),
                "::",
                stringify!(button)
            )
        );
    }
    test_field_button();
    fn test_field_next() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_code>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_code),
                "::",
                stringify!(next)
            )
        );
    }
    test_field_next();
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lirc_config_raw {
    pub lircrc_class: *mut ::std::os::raw::c_char,
    pub current_mode: *mut ::std::os::raw::c_char,
    pub next: *mut lirc_config_entry,
    pub first: *mut lirc_config_entry,
    pub sockfd: ::std::os::raw::c_int,
}

#[test]
fn bindgen_test_layout_lirc_config() {
    assert_eq!(
        ::std::mem::size_of::<lirc_config_raw>(),
        20usize,
        concat!("Size of: ", stringify!(lirc_config_raw))
    );
    assert_eq!(
        ::std::mem::align_of::<lirc_config_raw>(),
        4usize,
        concat!("Alignment of ", stringify!(lirc_config_raw))
    );
    fn test_field_lircrc_class() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_raw>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).lircrc_class) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_raw),
                "::",
                stringify!(lircrc_class)
            )
        );
    }
    test_field_lircrc_class();
    fn test_field_current_mode() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_raw>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).current_mode) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_raw),
                "::",
                stringify!(current_mode)
            )
        );
    }
    test_field_current_mode();
    fn test_field_next() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_raw>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_raw),
                "::",
                stringify!(next)
            )
        );
    }
    test_field_next();
    fn test_field_first() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_raw>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).first) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_raw),
                "::",
                stringify!(first)
            )
        );
    }
    test_field_first();
    fn test_field_sockfd() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_raw>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sockfd) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_raw),
                "::",
                stringify!(sockfd)
            )
        );
    }
    test_field_sockfd();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lirc_config_entry {
    pub prog: *mut ::std::os::raw::c_char,
    pub code: *mut lirc_code,
    pub rep_delay: ::std::os::raw::c_uint,
    pub ign_first_events: ::std::os::raw::c_uint,
    pub rep: ::std::os::raw::c_uint,
    pub config: *mut lirc_list,
    pub change_mode: *mut ::std::os::raw::c_char,
    pub flags: ::std::os::raw::c_uint,
    pub mode: *mut ::std::os::raw::c_char,
    pub next_config: *mut lirc_list,
    pub next_code: *mut lirc_code,
    pub next: *mut lirc_config_entry,
}
#[test]
fn bindgen_test_layout_lirc_config_entry() {
    assert_eq!(
        ::std::mem::size_of::<lirc_config_entry>(),
        48usize,
        concat!("Size of: ", stringify!(lirc_config_entry))
    );
    assert_eq!(
        ::std::mem::align_of::<lirc_config_entry>(),
        4usize,
        concat!("Alignment of ", stringify!(lirc_config_entry))
    );
    fn test_field_prog() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_entry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).prog) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_entry),
                "::",
                stringify!(prog)
            )
        );
    }
    test_field_prog();
    fn test_field_code() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_entry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).code) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_entry),
                "::",
                stringify!(code)
            )
        );
    }
    test_field_code();
    fn test_field_rep_delay() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_entry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rep_delay) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_entry),
                "::",
                stringify!(rep_delay)
            )
        );
    }
    test_field_rep_delay();
    fn test_field_ign_first_events() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_entry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ign_first_events) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_entry),
                "::",
                stringify!(ign_first_events)
            )
        );
    }
    test_field_ign_first_events();
    fn test_field_rep() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_entry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rep) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_entry),
                "::",
                stringify!(rep)
            )
        );
    }
    test_field_rep();
    fn test_field_config() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_entry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).config) as usize - ptr as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_entry),
                "::",
                stringify!(config)
            )
        );
    }
    test_field_config();
    fn test_field_change_mode() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_entry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).change_mode) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_entry),
                "::",
                stringify!(change_mode)
            )
        );
    }
    test_field_change_mode();
    fn test_field_flags() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_entry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize
            },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_entry),
                "::",
                stringify!(flags)
            )
        );
    }
    test_field_flags();
    fn test_field_mode() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_entry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mode) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_entry),
                "::",
                stringify!(mode)
            )
        );
    }
    test_field_mode();
    fn test_field_next_config() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_entry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).next_config) as usize - ptr as usize
            },
            36usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_entry),
                "::",
                stringify!(next_config)
            )
        );
    }
    test_field_next_config();
    fn test_field_next_code() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_entry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).next_code) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_entry),
                "::",
                stringify!(next_code)
            )
        );
    }
    test_field_next_code();
    fn test_field_next() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_config_entry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize
            },
            44usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_config_entry),
                "::",
                stringify!(next)
            )
        );
    }
    test_field_next();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lirc_cmd_ctx {
    pub packet: [::std::os::raw::c_char; 257usize],
    pub buffer: [::std::os::raw::c_char; 257usize],
    pub reply: [::std::os::raw::c_char; 257usize],
    pub head: ::std::os::raw::c_int,
    pub reply_to_stdout: ::std::os::raw::c_int,
    pub next: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_lirc_cmd_ctx() {
    assert_eq!(
        ::std::mem::size_of::<lirc_cmd_ctx>(),
        784usize,
        concat!("Size of: ", stringify!(lirc_cmd_ctx))
    );
    assert_eq!(
        ::std::mem::align_of::<lirc_cmd_ctx>(),
        4usize,
        concat!("Alignment of ", stringify!(lirc_cmd_ctx))
    );
    fn test_field_packet() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_cmd_ctx>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).packet) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_cmd_ctx),
                "::",
                stringify!(packet)
            )
        );
    }
    test_field_packet();
    fn test_field_buffer() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_cmd_ctx>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).buffer) as usize - ptr as usize
            },
            257usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_cmd_ctx),
                "::",
                stringify!(buffer)
            )
        );
    }
    test_field_buffer();
    fn test_field_reply() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_cmd_ctx>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).reply) as usize - ptr as usize
            },
            514usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_cmd_ctx),
                "::",
                stringify!(reply)
            )
        );
    }
    test_field_reply();
    fn test_field_head() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_cmd_ctx>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).head) as usize - ptr as usize
            },
            772usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_cmd_ctx),
                "::",
                stringify!(head)
            )
        );
    }
    test_field_head();
    fn test_field_reply_to_stdout() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_cmd_ctx>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).reply_to_stdout) as usize - ptr as usize
            },
            776usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_cmd_ctx),
                "::",
                stringify!(reply_to_stdout)
            )
        );
    }
    test_field_reply_to_stdout();
    fn test_field_next() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lirc_cmd_ctx>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize
            },
            780usize,
            concat!(
                "Offset of field: ",
                stringify!(lirc_cmd_ctx),
                "::",
                stringify!(next)
            )
        );
    }
    test_field_next();
}

extern "C" {
    pub fn lirc_init(
        prog: *const ::std::os::raw::c_char,
        verbose: u32,
    ) -> i32;

    pub fn lirc_deinit() -> ::std::os::raw::c_int;

    pub fn lirc_readconfig(
        path: *const ::std::os::raw::c_char,
        config: *mut *mut lirc_config_raw,
        check: ::std::option::Option<
            unsafe extern "C" fn(s: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn lirc_freeconfig(config: *mut lirc_config_raw);
}
extern "C" {
    pub fn lirc_nextir() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn lirc_ir2char(
        config: *mut lirc_config_raw,
        code: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn lirc_nextcode(code: *mut *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lirc_code2char(
        config: *mut lirc_config_raw,
        code: *mut ::std::os::raw::c_char,
        string: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lirc_readconfig_only(
        file: *const ::std::os::raw::c_char,
        config: *mut *mut lirc_config_raw,
        check: ::std::option::Option<
            unsafe extern "C" fn(s: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lirc_code2charprog(
        config: *mut lirc_config_raw,
        code: *mut ::std::os::raw::c_char,
        string: *mut *mut ::std::os::raw::c_char,
        prog: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lirc_getsocketname(
        id: *const ::std::os::raw::c_char,
        buf: *mut ::std::os::raw::c_char,
        size: size_t,
    ) -> size_t;
}
extern "C" {
    pub fn lirc_getmode(config: *mut lirc_config_raw) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lirc_setmode(
        config: *mut lirc_config_raw,
        mode: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn lirc_command_init(
        ctx: *mut lirc_cmd_ctx,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lirc_command_run(
        ctx: *mut lirc_cmd_ctx,
        fd: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lirc_command_reply_to_stdout(ctx: *mut lirc_cmd_ctx);
}
extern "C" {
    pub fn lirc_send_one(
        fd: ::std::os::raw::c_int,
        remote: *const ::std::os::raw::c_char,
        keysym: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lirc_simulate(
        fd: ::std::os::raw::c_int,
        remote: *const ::std::os::raw::c_char,
        keysym: *const ::std::os::raw::c_char,
        scancode: ::std::os::raw::c_int,
        repeat: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lirc_get_remote_socket(
        address: *const ::std::os::raw::c_char,
        port: ::std::os::raw::c_int,
        quiet: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lirc_get_local_socket(
        path: *const ::std::os::raw::c_char,
        quiet: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
