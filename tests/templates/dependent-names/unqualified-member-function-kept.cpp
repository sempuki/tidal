// tidal-rules: DEP-14
// tidal-expect: accept
// tidal-note: At the definition, f finds Fixed::f. Every specialization keeps it, also Both<Extra>, whose dependent base Extra has an f too.
struct Fixed { static constexpr int f() { return 1; } };
struct Extra { static constexpr int f() { return 2; } };
template <class T> struct Both : Fixed, T {
  static constexpr int get() { return f(); }
};
static_assert(Both<Extra>::get() == 1);
