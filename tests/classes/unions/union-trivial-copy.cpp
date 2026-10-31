// tidal-rules: UNI-3
// tidal-expect: run
union U { int i; float f; };
int main() { U a; a.i = 9; U b(a); U c; c = a; return b.i == 9 && c.i == 9 ? 0 : 1; }
