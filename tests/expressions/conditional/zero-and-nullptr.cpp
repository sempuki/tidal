// tidal-rules: COND-3
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-note: Q-0102. GCC rejects a conditional with a literal 0 and nullptr. Clang and EDG accept it.
// tidal-note: Gap review 2026-10-31 (third).
static_assert(sizeof(true ? 0 : nullptr) == sizeof(nullptr));
auto p = true ? nullptr : 0;
