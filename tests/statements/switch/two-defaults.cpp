// tidal-rules: SW-3
// tidal-expect: reject
void f(int v) {
  switch (v) { default: break; default: break; }
}
