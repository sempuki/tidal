// tidal-rules: DEL-3
// tidal-expect: reject
void f() = delete("use g");
void g() { f(); }
