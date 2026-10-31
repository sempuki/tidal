// tidal-rules: ATTR-17
// tidal-expect: reject
// tidal-error-clang: consecutive left square brackets when introducing an attribute
// tidal-error-gcc: two consecutive .\[. shall only introduce an attribute
// tidal-error-edg: two consecutive left square brackets always introduce an attribute list
// tidal-note: Gap review 2026-10-31.
int arr[3];
int f() { return arr[[] { return 1; }()]; }
