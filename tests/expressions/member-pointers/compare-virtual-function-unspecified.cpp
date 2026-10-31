// tidal-rules: MPTR-7
// tidal-expect: run
struct V { virtual void f() {} virtual void g() {} };
int main() { void (V::* p)() = &V::f; void (V::* q)() = &V::g; bool r = (p == q); return r || !r ? 0 : 1; }
