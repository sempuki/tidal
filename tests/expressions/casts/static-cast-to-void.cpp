// tidal-rules: CAST-1
// tidal-expect: accept
int f();
void g() { static_cast<void>(f()); }
