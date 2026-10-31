// tidal-rules: LIST-9
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: no viable conversion from .int. to .In
// tidal-error-gcc: could not convert .1. from .int. to .In
// tidal-note: Gap review 2026-10-30. Q-0043. EDG applies brace elision to parenthesized aggregate initialization; Clang and GCC reject it.
struct In { int a, b; };
struct Out { In i; int c; };
Out o(1, 2, 3);
