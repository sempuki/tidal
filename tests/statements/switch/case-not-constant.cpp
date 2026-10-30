// tidal-rules: SW-3
// tidal-expect: reject
void f(int v, int k) {
  switch (v) { case k: break; }
}
