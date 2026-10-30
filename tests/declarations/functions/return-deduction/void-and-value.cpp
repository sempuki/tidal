// tidal-rules: RET-1
// tidal-expect: reject
auto f(bool c) { if (c) return; return 1; }
