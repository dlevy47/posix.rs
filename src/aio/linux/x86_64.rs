#[repr(C)]
pub struct aiocb {
    pub aio_fildes: ::int_t,
    pub aio_lio_opcode: ::int_t,
    pub aio_reqprio: ::int_t,
    pub aio_buf: *mut ::void_t,
    pub aio_nbytes: ::size_t,
    pub aio_sigevent: ::signal::sigevent,
    __next_prio: *mut aiocb,
    __abs_prio: ::int_t,
    __policy: ::int_t,
    __error_code: ::int_t,
    __return_value: ::ssize_t,
    pub aio_offset: ::sys::types::off_t,
    __pad: [::char_t, ..0u],
    __glibc_reserved: [::char_t, ..32u],
}

pub static AIO_CANCELED:    ::uint_t = 0;
pub static AIO_NOTCANCELED: ::uint_t = 1;
pub static AIO_ALLDONE:     ::uint_t = 2;
pub static LIO_READ:        ::uint_t = 0;
pub static LIO_WRITE:       ::uint_t = 1;
pub static LIO_NOP:         ::uint_t = 2;
pub static LIO_WAIT:        ::uint_t = 0;
pub static LIO_NOWAIT:      ::uint_t = 1;
