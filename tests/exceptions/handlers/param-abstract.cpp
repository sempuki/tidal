// tidal-rules: TRY-5
// tidal-expect: reject
struct A { virtual void f() = 0; };
void g() { try { } catch (A) { } }
