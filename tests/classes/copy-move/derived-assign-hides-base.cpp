// tidal-rules: CPY-22
// tidal-expect: reject
// tidal-error-clang: no viable overloaded .=
// tidal-error-gcc: no match for .operator=
// tidal-error-edg: no operator .=. matches
// tidal-note: Gap review 2026-10-31.
struct B { B& operator=(int) { return *this; } };
struct D : B {};
void f(D& d) { d = 1; }
