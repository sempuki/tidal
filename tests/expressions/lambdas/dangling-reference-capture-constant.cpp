// tidal-rules: LAM-19
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
constexpr auto make() { int local = 1; return [&local] { return local; }; }
constexpr int v = make()();
