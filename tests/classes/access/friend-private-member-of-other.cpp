// tidal-rules: ACC-15
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
class Other { void hidden(); };
class C { friend void Other::hidden(); };
