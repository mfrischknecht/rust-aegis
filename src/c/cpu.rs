use core::ffi::c_int;

pub use crate::Error;

extern "C" {
    fn aegis_runtime_get_cpu_features() -> c_int;
    fn aegis_runtime_has_neon() -> c_int;
    fn aegis_runtime_has_armcrypto() -> c_int;
    fn aegis_runtime_has_avx() -> c_int;
    fn aegis_runtime_has_avx2() -> c_int;
    fn aegis_runtime_has_avx512f() -> c_int;
    fn aegis_runtime_has_aesni() -> c_int;
    fn aegis_runtime_has_vaes() -> c_int;
}

pub fn get_cpu_features() -> i32 { unsafe { aegis_runtime_get_cpu_features() } }
pub fn has_neon() -> i32 { unsafe { aegis_runtime_has_neon() } }
pub fn has_armcrypto() -> i32 { unsafe { aegis_runtime_has_armcrypto() } }
pub fn has_avx() -> i32 { unsafe { aegis_runtime_has_avx() } }
pub fn has_avx2() -> i32 { unsafe { aegis_runtime_has_avx2() } }
pub fn has_avx512f() -> i32 { unsafe { aegis_runtime_has_avx512f() } }
pub fn has_aesni() -> i32 { unsafe { aegis_runtime_has_aesni() } }
pub fn has_vaes() -> i32 { unsafe { aegis_runtime_has_vaes() } }