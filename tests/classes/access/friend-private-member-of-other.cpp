// tidal-rules: ACC-15
// tidal-expect: reject
// tidal-error-clang: friend function .* is a private member
// tidal-error-gcc: (private|inaccessible) within this context
// tidal-error-edg: is inaccessible
// tidal-note: Gap review 2026-10-31.
class Other { void hidden(); };
class C { friend void Other::hidden(); };
