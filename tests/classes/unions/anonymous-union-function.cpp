// tidal-rules: UNI-7
// tidal-expect: reject
void f() { union { int i; void g(); }; }
