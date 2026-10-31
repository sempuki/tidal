// tidal-rules: STMT-5
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: expected primary-expression before .\}
// tidal-note: Q-0096. GCC rejects a default label as the last item of a block. Clang and EDG accept it.
// tidal-note: Gap review 2026-10-31 (third).
int f(int v) { switch (v) { case 1: break; default: } return v; }
