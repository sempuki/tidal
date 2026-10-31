// tidal-rules: CPY-6
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
struct B { B& operator=(int) { return *this; } };
struct D : B {};
void f(D& d) { d = 1; }
