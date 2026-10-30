// tidal-rules: FN-3
// tidal-expect: run
int calls = 0;
void f() { ++calls; }
int main() { f(); return calls == 1 ? 0 : 1; }
