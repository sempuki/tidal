// tidal-rules: EXS-4
// tidal-expect: accept
void g(int* p) { static_assert(noexcept(delete p)); }
