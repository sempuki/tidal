// tidal-rules: CLS-4
// tidal-expect: run
struct S { int get() const; int v = 5; };
int S::get() const { return v; }
int main() { return S{}.get() == 5 ? 0 : 1; }
