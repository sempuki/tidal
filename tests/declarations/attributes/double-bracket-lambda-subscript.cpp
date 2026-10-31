// tidal-rules: ATTR-17
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
int arr[3];
int f() { return arr[[] { return 1; }()]; }
