// tidal-rules: DCL-5
// tidal-expect: reject
// tidal-error-clang: needs an explicit size or an initializer
// tidal-error-gcc: storage size of .a. isn.t known
// tidal-error-edg: incomplete type .int \[\]. is not allowed
void f() {
  int a[];
}
