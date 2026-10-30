// tidal-rules: CMP-8
// tidal-expect: accept
void f(); void g();
bool lt() { return (&f) < (&g); }
