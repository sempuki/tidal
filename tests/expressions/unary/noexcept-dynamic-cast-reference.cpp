// tidal-rules: UN-18
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
struct B { virtual ~B() {} };
struct D : B {};
void f(B& b, B* p) { static_assert(!noexcept(dynamic_cast<D&>(b))); static_assert(noexcept(dynamic_cast<D*>(p))); }
