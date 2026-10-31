// tidal-rules: UNI-1
// tidal-expect: run
union U { char c; int i; double d; };
int main() { U u; return static_cast<void*>(&u.c) == static_cast<void*>(&u.d) && static_cast<void*>(&u) == static_cast<void*>(&u.i) ? 0 : 1; }
