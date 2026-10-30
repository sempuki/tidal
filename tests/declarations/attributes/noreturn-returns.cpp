// tidal-rules: ATTR-3
// tidal-expect: reject
// tidal-flags: -Werror
[[noreturn]] void stop(int code) {
  (void)code;
}
