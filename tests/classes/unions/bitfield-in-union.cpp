// tidal-rules: BF-1
// tidal-expect: run
union U { unsigned x : 4; unsigned y; };
int main() { U u{}; u.x = 3; return u.x == 3 ? 0 : 1; }
