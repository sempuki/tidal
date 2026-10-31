// tidal-rules: ATTR-6
// tidal-expect: accept
// tidal-flags: -Werror -Wunused
struct Guard { Guard(); };
struct Closer { ~Closer(); };
void f() {
  Guard guard;
  Closer closer;
}
