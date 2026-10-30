// tidal-rules: CMP-19
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
int* p = nullptr;
int** pp = &p;
const int* const* cq = nullptr;
const int** q = nullptr;
bool a = pp == cq;
bool b = pp == q;
