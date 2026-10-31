// tidal-rules: LAM-10
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
auto l = [](int v) noexcept { return v; };
int (*p)(int) noexcept = l;
