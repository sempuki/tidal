// tidal-rules: ATTR-6
// tidal-expect: accept
// tidal-flags: -Werror -Wunused
struct Guard { Guard(); };
struct Holder { int count; Guard guard; };
void f() {
  Holder holder;
}
