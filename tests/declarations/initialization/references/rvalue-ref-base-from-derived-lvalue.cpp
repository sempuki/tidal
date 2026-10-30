// tidal-rules: REF-3
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
struct Base {};
struct Derived : Base {};
Derived d;
Base&& r = d;
