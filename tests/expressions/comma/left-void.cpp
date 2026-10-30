// tidal-rules: COMMA-2
// tidal-expect: accept
void f();
int g() { return (f(), 4); }
