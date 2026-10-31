// tidal-rules: AUTO-15
// tidal-expect: reject
// tidal-error-clang: array initializer must be an initializer list
// tidal-error-gcc: array must be initialized with a brace-enclosed initializer
// tidal-error-edg: initialization with .\{\.\.\.\}. expected for aggregate
// tidal-note: Gap review 2026-10-31.
int arr[3] = {1, 2, 3};
decltype(auto) copy = arr;
