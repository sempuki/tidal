// tidal-rules: TRY-7
// tidal-expect: reject
void g(int n) { switch (n) { try { } catch (...) { case 1: ; } } }
