// tidal-rules: ATTR-6
// tidal-expect: accept
// tidal-flags: -Werror -Wunused
struct Guard { Guard(); };
struct Pair { Guard guards[2]; };
struct Holder { Pair pair; };
void f() {
  Holder holder;
}
