// tidal-rules: VC-3
// tidal-expect: reject
int x = 0;
int* p = &static_cast<int&&>(x);
