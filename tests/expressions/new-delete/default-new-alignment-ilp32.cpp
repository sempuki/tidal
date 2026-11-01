// tidal-rules: NEW-27
// tidal-expect: accept
// tidal-flags: -m32
// tidal-expect-clang: reject
// tidal-error-clang: static assertion failed
// tidal-note: The default new alignment is implementation-defined (NEW-27). Clang gives 8 on -m32, and GCC and EDG give 16.
static_assert(__STDCPP_DEFAULT_NEW_ALIGNMENT__ == 16);
