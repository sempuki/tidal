// tidal-rules: FN-4
// tidal-expect: reject
// tidal-error-clang: main. must return .int
// tidal-error-gcc: main. must return .int
// tidal-error-edg: main. must be .int
// tidal-note: Gap review 2026-10-30.
auto main() { return 0; }
