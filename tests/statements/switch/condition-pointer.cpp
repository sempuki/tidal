// tidal-rules: SW-2
// tidal-expect: reject
void f(int* p) {
  switch (p) { default: break; }
}
