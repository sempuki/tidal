// tidal-rules: CMP-15
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Q-0104. EDG evaluates the address comparison of a struct and its first member to false. Clang and GCC give true.
// tidal-note: Gap review 2026-10-31 (third).
struct S { int first; int second; };
constexpr S s{};
static_assert(static_cast<const void*>(&s) == static_cast<const void*>(&s.first));
