// tidal-rules: INST-14
// tidal-expect: accept
// tidal-note: Each specialization of depth instantiates the next one, 50 levels deep. Every compiler in the matrix allows that depth. EDG stops at 64 levels of function templates.
template <int N> constexpr int depth() { return depth<N - 1>() + 1; }
template <> constexpr int depth<0>() { return 0; }
static_assert(depth<50>() == 50);
