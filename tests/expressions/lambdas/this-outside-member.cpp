// tidal-rules: LAM-8
// tidal-expect: reject
void f() {
  auto g = [this] { return 1; };
}
