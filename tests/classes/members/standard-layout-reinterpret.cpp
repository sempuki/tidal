// tidal-rules: CLS-19
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct S { int a; int b; };
int main() { S s{4, 5}; int* p = reinterpret_cast<int*>(&s); S* q = reinterpret_cast<S*>(p); return *p == 4 && q->b == 5 ? 0 : 1; }
