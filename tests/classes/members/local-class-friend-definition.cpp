// tidal-rules: CLS-28
// tidal-expect: reject
// tidal-error-clang: no matching function found in local scope|cannot define friend function in a local class
// tidal-error-gcc: cannot define friend function .h. in a local class definition
// tidal-error-edg: local-class friend declaration|cannot be defined in the current scope
// tidal-note: Gap review 2026-10-31.
void g() { struct L { friend void h() {} }; }
