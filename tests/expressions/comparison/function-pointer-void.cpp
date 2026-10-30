// tidal-rules: CMP-9
// tidal-expect: reject
void f();
bool eq(void* p) { return &f == p; }
