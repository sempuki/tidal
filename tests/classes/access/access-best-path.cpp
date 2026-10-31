// tidal-rules: ACC-8
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct V { int x = 3; };
struct A : private virtual V {};
struct B : public virtual V {};
struct D : A, B {};
int main() { D d; return d.x == 3 ? 0 : 1; }
