// tidal-rules: CLS-9
// tidal-expect: reject
// tidal-error-clang: static data member .* not allowed in anonymous struct
// tidal-error-gcc: static data member .* in unnamed class
// tidal-error-edg: static data member declaration is not allowed in this class
// tidal-note: Gap review 2026-10-31.
struct { struct In { static int n; } in; } x;
