// tidal-rules: TYN-4
// tidal-expect: reject
void f() { struct Local* p = nullptr; (void)p; }
Local* q;
