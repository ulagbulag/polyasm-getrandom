#[cfg(any(
    getrandom_backend = "linux_getrandom",
    target_os = "aix",
    target_os = "android",
    target_os = "cygwin",
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "freebsd",
    target_os = "haiku",
    all(target_os = "horizon", target_arch = "arm"),
    target_os = "hurd",
    target_os = "illumos",
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "nto",
    target_os = "openbsd",
    target_os = "redox",
    target_os = "vita",
))]
pub(crate) mod get_errno;

#[cfg(any(
    getrandom_backend = "efi_rng",
    target_os = "android",
    target_os = "linux",
    target_os = "netbsd",
))]
pub(crate) mod lazy_ptr;

#[cfg(any(
    getrandom_backend = "linux_getrandom",
    getrandom_backend = "linux_raw",
    target_os = "aix",
    target_os = "android",
    target_os = "cygwin",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "haiku",
    all(target_os = "horizon", target_arch = "arm"),
    target_os = "hurd",
    target_os = "illumos",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "nto",
    target_os = "redox",
))]
pub(crate) mod sanitizer;

#[cfg(any(
    getrandom_backend = "linux_getrandom",
    target_os = "aix",
    target_os = "android",
    target_os = "cygwin",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "haiku",
    all(target_os = "horizon", target_arch = "arm"),
    target_os = "hurd",
    target_os = "illumos",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "nto",
    target_os = "redox",
))]
pub(crate) mod sys_fill_exact;
