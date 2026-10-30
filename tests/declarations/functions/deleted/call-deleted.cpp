// tidal-rules: DEL-1
// tidal-expect: reject
void f() = delete;
void g() { f(); }
