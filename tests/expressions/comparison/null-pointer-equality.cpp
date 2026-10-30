// tidal-rules: CMP-19
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
static_assert(nullptr == nullptr);
struct S { int v; };
constexpr int S::* a = nullptr;
constexpr int S::* b = nullptr;
static_assert(a == b);
static_assert(&S::v != a);
