// tidal-rules: BF-1
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: non-integral type
// tidal-note: G151, Q-0314. W has an explicit conversion to int and a non-explicit conversion to long. Only the non-explicit one counts, so the width converts to 4.
struct W {
  constexpr explicit operator int() const { return 3; }
  constexpr operator long() const { return 4; }
};
struct S { unsigned f : W{}; };
constexpr S s{15};
static_assert(s.f == 15);
