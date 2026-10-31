// tidal-rules: UNI-4
// tidal-expect: run
union U { int i; float f; };
int main() { U u = {5}; return u.i == 5 ? 0 : 1; }
