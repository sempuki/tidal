// tidal-rules: LOG-12
// tidal-expect: accept
// tidal-expect-gcc-14: reject
// tidal-error-gcc: nullptr_t.*requires direct-initialization
// tidal-note: Q-0101. GCC 14 rejects nullptr as an operand of a logical operator. GCC 16, Clang, and EDG accept it.
// tidal-note: Gap review 2026-10-31 (third).
static_assert(!nullptr);
static_assert(!(nullptr && true));
static_assert(nullptr || true);
