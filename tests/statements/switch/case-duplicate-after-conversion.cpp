// tidal-rules: SW-3
// tidal-expect: reject
void f(int v) {
  switch (v) { case 1: break; case '\x01': break; }
}
