// tidal-rules: INIT-5
// tidal-expect: reject
// tidal-error-clang: array initializer must be an initializer list
// tidal-error-gcc: array must be initialized with a brace-enclosed initializer
// tidal-error-edg: initialization with .\{\.\.\.\}. expected for aggregate
int a[2] = {1, 2};
int b[2] = a;
