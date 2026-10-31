// tidal-rules: PRIM-15
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-note: Q-0063. Clang and GCC accept with a warning and give infinity. EDG 7.0 rejects it.
float x = 1e999f;
