// tidal-rules: CAST-15
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct B { int b = 1; };
struct D : B { int d = 2; };
int main() { D obj; B* p = &obj; D* q = static_cast<D*>(p); B& r = obj; return q->d == 2 && static_cast<D&>(r).d == 2 ? 0 : 1; }
