// tidal-rules: CE-17
// tidal-expect: accept
// tidal-note: The evaluations reach a static constexpr variable, a static const int with a constant initializer, and a thread_local constexpr variable. Each is usable in constant expressions, so CE-17 allows it.
constexpr int fixed() { static constexpr int kept = 3; return kept; }
constexpr int counted() { static const int kept = 4; return kept; }
constexpr int local() { thread_local constexpr int kept = 5; return kept; }
constexpr const int* address() { static const int kept = 6; return &kept; }
static_assert(fixed() == 3 && counted() == 4 && local() == 5);
static_assert(*address() == 6);
