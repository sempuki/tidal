// tidal-rules: NEW-8
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main() { const int* p = new const int(2); delete p; const int* q = new const int[2]{}; delete[] q; return 0; }
