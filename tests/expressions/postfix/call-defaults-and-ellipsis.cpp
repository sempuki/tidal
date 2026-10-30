// tidal-rules: PFX-5
// tidal-expect: accept
int f(int a, int b = 2);
int v(int a, ...);
int g() { return f(1) + v(1, 2, 3); }
