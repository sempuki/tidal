// tidal-rules: NAM-17
// tidal-expect: accept
struct base { static constexpr int value = 1; using type = int; };
struct derived : base { };
static_assert(derived::value == 1);
derived::type t = 0;
