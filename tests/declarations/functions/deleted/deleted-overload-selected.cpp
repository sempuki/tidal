// tidal-rules: DEL-1
// tidal-expect: reject
void f(int);
void f(double) = delete;
void g() { f(1.5); }
