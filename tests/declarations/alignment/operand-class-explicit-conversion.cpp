// tidal-rules: ALN-8
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: non-integral type
// tidal-note: G151, Q-0314. W has an explicit conversion to int and a non-explicit conversion to long. Only the non-explicit one counts, so the operand converts to 16.
struct W {
  constexpr explicit operator int() const { return 8; }
  constexpr operator long() const { return 16; }
};
constexpr W w;
struct alignas(w) S { char c; };
static_assert(alignof(S) == 16);
