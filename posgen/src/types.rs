static TYPES: ::phf::PhfMap<&'static str, &'static str> = phf_map! {
    "utmpx"                      => "::utmpx::utmpx",
    "group"                      => "::grp::group",
    "t_scalar_t"                 => "::stropts::t_scalar_t",
    "t_uscalar_t"                => "::stropts::t_uscalar_t",
    "bandinfo"                   => "::stropts::bandinfo",
    "strbuf"                     => "::stropts::strbuf",
    "strpeek"                    => "::stropts::strpeek",
    "strfdinsert"                => "::stropts::strfdinsert",
    "strioctl"                   => "::stropts::strioctl",
    "strrecvfd"                  => "::stropts::strrecvfd",
    "str_mlist"                  => "::stropts::str_mlist",
    "str_list"                   => "::stropts::str_list",
    "timespec"                   => "::time::timespec",
    "tm"                         => "::time::tm",
    "itimerspec"                 => "::time::itimerspec",
    "posix_spawnattr_t"          => "::spawn::posix_spawnattr_t",
    "posix_spawn_file_actions_t" => "::spawn::posix_spawn_file_actions_t",
    "datum"                      => "::ndbm::datum",
    "DBM"                        => "::ndbm::DBM",
    "passwd"                     => "::pwd::passwd",
    "glob_t"                     => "::glob::glob_t",
    "dirent"                     => "::dirent::dirent",
    "DIR"                        => "::dirent::DIR",
    "fexcept_t"                  => "::fenv::fexcept_t",
    "fenv_t"                     => "::fenv::fenv_t",
    "flock"                      => "::fcntl::flock",
    "nl_catd"                    => "::nl_types::nl_catd",
    "nl_item"                    => "::nl_types::nl_item",
    "imaxdiv_t"                  => "::inttypes::imaxdiv_t",
    "nfds_t"                     => "::poll::nfds_t",
    "pollfd"                     => "::poll::pollfd",
    "FILE"                       => "::stdio::FILE",
    "fpos_t"                     => "::stdio::fpos_t",
    "sched_param"                => "::sched::sched_param",
    "int8_t"                     => "::stdint::int8_t",
    "int16_t"                    => "::stdint::int16_t",
    "int32_t"                    => "::stdint::int32_t",
    "int64_t"                    => "::stdint::int64_t",
    "uint8_t"                    => "::stdint::uint8_t",
    "uint16_t"                   => "::stdint::uint16_t",
    "uint32_t"                   => "::stdint::uint32_t",
    "uint64_t"                   => "::stdint::uint64_t",
    "int_least8_t"               => "::stdint::int_least8_t",
    "int_least16_t"              => "::stdint::int_least16_t",
    "int_least32_t"              => "::stdint::int_least32_t",
    "int_least64_t"              => "::stdint::int_least64_t",
    "uint_least8_t"              => "::stdint::uint_least8_t",
    "uint_least16_t"             => "::stdint::uint_least16_t",
    "uint_least32_t"             => "::stdint::uint_least32_t",
    "uint_least64_t"             => "::stdint::uint_least64_t",
    "int_fast8_t"                => "::stdint::int_fast8_t",
    "int_fast16_t"               => "::stdint::int_fast16_t",
    "int_fast32_t"               => "::stdint::int_fast32_t",
    "int_fast64_t"               => "::stdint::int_fast64_t",
    "uint_fast8_t"               => "::stdint::uint_fast8_t",
    "uint_fast16_t"              => "::stdint::uint_fast16_t",
    "uint_fast32_t"              => "::stdint::uint_fast32_t",
    "uint_fast64_t"              => "::stdint::uint_fast64_t",
    "intptr_t"                   => "::stdint::intptr_t",
    "uintptr_t"                  => "::stdint::uintptr_t",
    "intmax_t"                   => "::stdint::intmax_t",
    "uintmax_t"                  => "::stdint::uintmax_t",
    "div_t"                      => "::stdlib::div_t",
    "ldiv_t"                     => "::stdlib::ldiv_t",
    "lldiv_t"                    => "::stdlib::lldiv_t",
    "jmp_buf"                    => "::setjmp::jmp_buf",
    "sigjmp_buf"                 => "::setjmp::sigjmp_buf",
    "entry"                      => "::search::entry",
    "utimbuf"                    => "::utime::utimbuf",
    "lconv"                      => "::locale::lconv",
    "locale_t"                   => "::locale::locale_t",
    "in_port_t"                  => "::netinet::_in::in_port_t",
    "in_addr_t"                  => "::netinet::_in::in_addr_t",
    "in_addr"                    => "::netinet::_in::in_addr",
    "in6_addr"                   => "::netinet::_in::in6_addr",
    "sockaddr_in"                => "::netinet::_in::sockaddr_in",
    "sockaddr_in6"               => "::netinet::_in::sockaddr_in6",
    "ipv6_mreq"                  => "::netinet::_in::ipv6_mreq",
    "timeval"                    => "::sys::time::timeval",
    "itimerval"                  => "::sys::time::itimerval",
    "sockaddr_un"                => "::sys::un::sockaddr_un",
    "tms"                        => "::sys::times::tms",
    "utsname"                    => "::sys::utsname::utsname",
    "fd_set"                     => "::sys::select::fd_set",
    "statvfs"                    => "::sys::statvfs::statvfs",
    "dev_t"                      => "::sys::types::dev_t",
    "uid_t"                      => "::sys::types::uid_t",
    "gid_t"                      => "::sys::types::gid_t",
    "ino_t"                      => "::sys::types::ino_t",
    "mode_t"                     => "::sys::types::mode_t",
    "nlink_t"                    => "::sys::types::nlink_t",
    "off_t"                      => "::sys::types::off_t",
    "pid_t"                      => "::sys::types::pid_t",
    "clock_t"                    => "::sys::types::clock_t",
    "id_t"                       => "::sys::types::id_t",
    "time_t"                     => "::sys::types::time_t",
    "suseconds_t"                => "::sys::types::suseconds_t",
    "key_t"                      => "::sys::types::key_t",
    "clockid_t"                  => "::sys::types::clockid_t",
    "timer_t"                    => "::sys::types::timer_t",
    "blksize_t"                  => "::sys::types::blksize_t",
    "blkcnt_t"                   => "::sys::types::blkcnt_t",
    "fsblkcnt_t"                 => "::sys::types::fsblkcnt_t",
    "fsfilcnt_t"                 => "::sys::types::fsfilcnt_t",
    "pthread_t"                  => "::sys::types::pthread_t",
    "pthread_attr_t"             => "::sys::types::pthread_attr_t",
    "pthread_mutex_t"            => "::sys::types::pthread_mutex_t",
    "pthread_mutexattr_t"        => "::sys::types::pthread_mutexattr_t",
    "pthread_cond_t"             => "::sys::types::pthread_cond_t",
    "pthread_condattr_t"         => "::sys::types::pthread_condattr_t",
    "pthread_key_t"              => "::sys::types::pthread_key_t",
    "pthread_once_t"             => "::sys::types::pthread_once_t",
    "pthread_rwlock_t"           => "::sys::types::pthread_rwlock_t",
    "pthread_rwlockattr_t"       => "::sys::types::pthread_rwlockattr_t",
    "pthread_spinlock_t"         => "::sys::types::pthread_spinlock_t",
    "pthread_barrier_t"          => "::sys::types::pthread_barrier_t",
    "pthread_barrierattr_t"      => "::sys::types::pthread_barrierattr_t",
    "semid_ds"                   => "::sys::sem::semid_ds",
    "sembuf"                     => "::sys::sem::sembuf",
    "socklen_t"                  => "::sys::socket::socklen_t",
    "sa_family_t"                => "::sys::socket::sa_family_t",
    "sockaddr"                   => "::sys::socket::sockaddr",
    "sockaddr_storage"           => "::sys::socket::sockaddr_storage",
    "msghdr"                     => "::sys::socket::msghdr",
    "cmsghdr"                    => "::sys::socket::cmsghdr",
    "linger"                     => "::sys::socket::linger",
    "ipc_perm"                   => "::sys::ipc::ipc_perm",
    "stat"                       => "::sys::stat::stat",
    "iovec"                      => "::sys::uio::iovec",
    "msgqnum_t"                  => "::sys::msg::msgqnum_t",
    "msglen_t"                   => "::sys::msg::msglen_t",
    "msqid_ds"                   => "::sys::msg::msqid_ds",
    "msginfo"                    => "::sys::msg::msginfo",
    "idtype_t"                   => "::sys::wait::idtype_t",
    "shmatt_t"                   => "::sys::shm::shmatt_t",
    "shmid_ds"                   => "::sys::shm::shmid_ds",
    "rlim_t"                     => "::sys::resource::rlim_t",
    "rlimit"                     => "::sys::resource::rlimit",
    "rusage"                     => "::sys::resource::rusage",
    "wordexp_t"                  => "::wordexp::wordexp_t",
    "mqd_t"                      => "::mqueue::mqd_t",
    "mq_attr"                    => "::mqueue::mq_attr",
    "wctrans_t"                  => "::wctype::wctrans_t",
    "ptrdiff_t"                  => "::stddef::ptrdiff_t",
    "wchar_t"                    => "::stddef::wchar_t",
    "iconv_t"                    => "::iconv::iconv_t",
    "aiocb"                      => "::aio::aiocb",
    "cc_t"                       => "::termios::cc_t",
    "speed_t"                    => "::termios::speed_t",
    "tcflag_t"                   => "::termios::tcflag_t",
    "termios"                    => "::termios::termios",
    "sig_atomic_t"               => "::signal::sig_atomic_t",
    "sigset_t"                   => "::signal::sigset_t",
    "sigevent"                   => "::signal::sigevent",
    "sigval"                     => "::signal::sigval",
    "sigaction"                  => "::signal::sigaction",
    "mcontext_t"                 => "::signal::mcontext_t",
    "ucontext"                   => "::signal::ucontext",
    "stack_t"                    => "::signal::stack_t",
    "siginfo_t"                  => "::signal::siginfo_t",
    "sem_t"                      => "::semaphore::sem_t",
    "regex_t"                    => "::regex::regex_t",
    "regoff_t"                   => "::regex::regoff_t",
    "regmatch_t"                 => "::regex::regmatch_t",
    "mbstate_t"                  => "::wchar::mbstate_t",
    "wint_t"                     => "::wchar::wint_t",
    "wctype_t"                   => "::wchar::wctype_t",
    "FTW"                        => "::ftw::FTW",
    "size_t"                     => "::size_t",
    "ssize_t"                    => "::ssize_t",
};

pub fn find(s: &str) -> Option<&'static str> {
    TYPES.find_equiv(&s).map(|v| *v)
}
