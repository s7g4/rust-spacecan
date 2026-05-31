// Mock implementations of defmt symbols to allow host unit tests to link successfully.
// These are only compiled when running tests (`cfg(test)`).

#[unsafe(no_mangle)]
extern "C" fn _defmt_acquire() {}

#[unsafe(no_mangle)]
extern "C" fn _defmt_release() {}

#[unsafe(no_mangle)]
extern "C" fn _defmt_write(_buf: *const u8, _len: usize) {}

#[unsafe(no_mangle)]
extern "C" fn _defmt_timestamp() -> u64 {
    0
}

#[unsafe(no_mangle)]
extern "C" fn _defmt_flush() {}

#[unsafe(no_mangle)]
static mut __DEFMT_MARKER_TRACE_START: u8 = 0;
#[unsafe(no_mangle)]
static mut __DEFMT_MARKER_TRACE_END: u8 = 0;
#[unsafe(no_mangle)]
static mut __DEFMT_MARKER_DEBUG_START: u8 = 0;
#[unsafe(no_mangle)]
static mut __DEFMT_MARKER_DEBUG_END: u8 = 0;
#[unsafe(no_mangle)]
static mut __DEFMT_MARKER_INFO_START: u8 = 0;
#[unsafe(no_mangle)]
static mut __DEFMT_MARKER_INFO_END: u8 = 0;
#[unsafe(no_mangle)]
static mut __DEFMT_MARKER_WARN_START: u8 = 0;
#[unsafe(no_mangle)]
static mut __DEFMT_MARKER_WARN_END: u8 = 0;
#[unsafe(no_mangle)]
static mut __DEFMT_MARKER_ERROR_START: u8 = 0;
#[unsafe(no_mangle)]
static mut __DEFMT_MARKER_ERROR_END: u8 = 0;
