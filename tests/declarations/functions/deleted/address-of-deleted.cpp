// tidal-rules: DEL-1
// tidal-expect: reject
void f() = delete;
auto p = &f;
