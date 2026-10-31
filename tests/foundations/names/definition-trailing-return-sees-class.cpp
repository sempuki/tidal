// tidal-rules: NAM-21
// tidal-expect: accept
struct C { using type = int; type get(); };
auto C::get() -> type { return 0; }
