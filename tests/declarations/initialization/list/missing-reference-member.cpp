// tidal-rules: LIST-12
// tidal-expect: reject
// tidal-error-clang: reference member of type .int &. uninitialized
// tidal-error-gcc: is uninitialized reference
// tidal-error-edg: uninitialized const or reference member
// tidal-note: Gap review 2026-10-30.
struct R { int a; int& r; };
R x{1};
