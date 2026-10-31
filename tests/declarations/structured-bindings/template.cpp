// tidal-rules: SB-7
// tidal-expect: reject
// tidal-error-clang: decomposition declaration template not supported|structured binding declaration cannot be a template
// tidal-error-gcc: expected unqualified-id before .\[. token
// tidal-error-edg: structured binding templates are not permitted
int a[2];
template <class T> auto [x, y] = a;
