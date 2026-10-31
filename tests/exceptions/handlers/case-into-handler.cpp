// tidal-rules: TRY-7
// tidal-expect: reject
// tidal-error-clang: cannot jump from switch statement to this case label
// tidal-error-gcc: jump to case label
// tidal-error-edg: transfer of control into an exception handler
void g(int n) { switch (n) { try { } catch (...) { case 1: ; } } }
