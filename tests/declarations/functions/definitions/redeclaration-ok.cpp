// tidal-rules: FN-2
// tidal-expect: run
int f();
int f();
int f() { return 0; }
int f();
int main() { return f(); }
