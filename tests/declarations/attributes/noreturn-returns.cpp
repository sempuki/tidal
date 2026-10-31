// tidal-rules: ATTR-14
// tidal-expect: reject
// tidal-error-clang: function declared .noreturn. should not return
// tidal-error-gcc: noreturn. function does return
// tidal-flags: -Werror
[[noreturn]] void stop(int code) {
  (void)code;
}
