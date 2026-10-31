// tidal-rules: TRY-5
// tidal-expect: accept
struct A { virtual void f() = 0; };
void g() { try { } catch (A&) { } }
