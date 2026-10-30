// tidal-rules: ATTR-6
// tidal-expect: accept
// tidal-flags: -Werror -Wunused
void f([[maybe_unused]] int p) {
  [[maybe_unused]] int local = 1;
}
