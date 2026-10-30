// tidal-rules: ATTR-1
// tidal-expect: accept
[[nodiscard, deprecated("old")]] int a();
[[nodiscard]] [[deprecated]] int b();
[[maybe_unused]] static int c = 0;
[[]] int d;
[[using gnu: unused]] static int e;
