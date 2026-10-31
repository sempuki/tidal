// tidal-rules: UNEV-2
// tidal-expect: run
int calls = 0;
int f() { return ++calls; }
int main() { f(); (void)f(); static_cast<void>(f()); return calls == 3 ? 0 : 1; }
