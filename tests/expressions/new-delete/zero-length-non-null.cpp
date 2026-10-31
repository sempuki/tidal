// tidal-rules: NEW-2
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main() { int n = 0; int* p = new int[n]; int* q = new int[n]; bool ok = p != nullptr && p != q; delete[] p; delete[] q; return ok ? 0 : 1; }
