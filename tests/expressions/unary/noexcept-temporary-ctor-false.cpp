// tidal-rules: UN-18
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct T { T() {} };
struct N { N() noexcept {} };
static_assert(!noexcept(T()));
static_assert(noexcept(N()));
