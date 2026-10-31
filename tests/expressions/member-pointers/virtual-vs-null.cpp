// tidal-rules: MPTR-5
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct V { virtual void f() {} };
int main() { void (V::* p)() = &V::f; return p != nullptr && !(p == nullptr) ? 0 : 1; }
