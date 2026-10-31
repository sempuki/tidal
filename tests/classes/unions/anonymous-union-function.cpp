// tidal-rules: UNI-9
// tidal-expect: reject
void f() { union { int i; void g(); }; }
