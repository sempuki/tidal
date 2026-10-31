// tidal-rules: CAST-1
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
enum class E { a, b };
E e = static_cast<E>(1.0);
