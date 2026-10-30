// tidal-rules: PFX-5
// tidal-expect: reject
int f(int a);
int g() { return f(1, 2); }
