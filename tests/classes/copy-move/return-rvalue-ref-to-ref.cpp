// tidal-rules: CPY-11
// tidal-expect: accept
struct M {};
M&& pass(M&& m) { return static_cast<M&&>(m); }
M& keep(M& m) { return m; }
