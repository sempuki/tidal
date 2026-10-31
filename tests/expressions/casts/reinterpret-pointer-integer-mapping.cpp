// tidal-rules: CAST-16
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main() { int x = 0; unsigned long v = reinterpret_cast<unsigned long>(&x); int* back = reinterpret_cast<int*>(v); return back == &x ? 0 : 1; }
