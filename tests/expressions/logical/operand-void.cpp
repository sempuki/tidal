// tidal-rules: LOG-3
// tidal-expect: reject
void g();
void f() {
  bool b = g() && true;
}
