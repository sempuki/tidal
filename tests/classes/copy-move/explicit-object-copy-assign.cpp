// tidal-rules: CPY-3
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int calls = 0;
struct S { int v = 0; S() {} S& operator=(this S& self, const S& o) { ++calls; self.v = o.v; return self; } };
int main() { S a, b; b.v = 4; a = b; return calls == 1 && a.v == 4 ? 0 : 1; }
