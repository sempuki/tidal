// tidal-rules: DEL-1
// tidal-expect: reject
// tidal-error-clang: call to deleted function
// tidal-error-gcc: use of deleted function .void f\(double\)
// tidal-error-edg: f\(double\). .* it is a deleted function
void f(int);
void f(double) = delete;
void g() { f(1.5); }
