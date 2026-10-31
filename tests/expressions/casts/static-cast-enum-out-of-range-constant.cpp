// tidal-rules: CAST-14
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: outside the valid range of values .* for the enumeration type
// tidal-note: Gap review 2026-10-31. Q-0017. GCC and EDG accept the out-of-range value during constant evaluation. Clang rejects it.
enum Small { a, b };
constexpr Small s = static_cast<Small>(9);
