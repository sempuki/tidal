// tidal-rules: ATTR-14
// tidal-expect: reject
// tidal-error-clang: function declared .noreturn. should not return
// tidal-error-gcc: noreturn. function does return
// tidal-flags: -Werror
// tidal-note: The function throws on one path and returns on the other.
[[noreturn]] void stop(bool fatal) {
  if (fatal) throw 1;
}
