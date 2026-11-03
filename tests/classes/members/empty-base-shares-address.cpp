// tidal-rules: CLS-10 CLS-19
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct E {};
struct D : E { int v; };
int main() { D d; return static_cast<void*>(static_cast<E*>(&d)) == static_cast<void*>(&d) ? 0 : 1; }
