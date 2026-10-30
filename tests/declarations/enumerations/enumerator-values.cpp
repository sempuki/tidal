// tidal-rules: ENUM-1
// tidal-expect: accept
enum E { A, B, C = 10, D, F = B + 1, G = C };
static_assert(A == 0 && B == 1 && C == 10 && D == 11 && F == 2 && G == 10);
