// tidal-rules: FN-4
// tidal-expect: reject
int f();
int main() { return 0; }
int f() { return main(); }
