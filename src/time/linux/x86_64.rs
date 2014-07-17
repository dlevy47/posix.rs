#[repr(C)]
pub struct timespec {
    pub tv_sec: ::sys::types::time_t,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}

#[repr(C)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}

pub static CLOCKS_PER_SEC: ::sys::types::clock_t = 1000000;

pub static CLOCK_MONOTONIC:          ::sys::types::clockid_t = 1;
pub static CLOCK_PROCESS_CPUTIME_ID: ::sys::types::clockid_t = 2;
pub static CLOCK_REALTIME:           ::sys::types::clockid_t = 0;
pub static CLOCK_THREAD_CPUTIME_ID:  ::sys::types::clockid_t = 3;

pub static TIMER_ABSTIME: ::uint_t = 1;

extern {
    pub static mut getdate_err: ::int_t;
}
