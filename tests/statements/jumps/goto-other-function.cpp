// tidal-rules: JMP-5
// tidal-expect: reject
void g() {
there:;
}
void f() {
  goto there;
}
