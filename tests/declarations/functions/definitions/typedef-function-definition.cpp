// tidal-rules: FN-12
// tidal-expect: reject
// tidal-error-edg: function type may not come from a typedef
// tidal-error-gcc: is initialized like a variable
// tidal-note: Gap review 2026-10-31.
typedef int Fn(int);
Fn twice { return 0; }
