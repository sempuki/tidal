// tidal-rules: LAM-19
// tidal-expect: reject
// tidal-error-gcc: accessing .local. outside its lifetime
// tidal-error-clang: read of variable whose lifetime has ended
// tidal-error-edg: attempt to access expired storage
// tidal-note: Gap review 2026-10-31.
constexpr auto make() { int local = 1; return [&local] { return local; }; }
constexpr int v = make()();
