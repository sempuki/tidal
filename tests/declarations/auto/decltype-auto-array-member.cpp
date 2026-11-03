// tidal-rules: AUTO-15
// tidal-expect: reject
// tidal-error-clang: array initializer must be an initializer list
// tidal-error-gcc: array must be initialized with a brace-enclosed initializer
// tidal-error-edg: initialization with .\{\.\.\.\}. expected for aggregate
// tidal-note: An unparenthesized member access to an array member gives the array type of the member.
struct S { int values[2]; };
S s{{1, 2}};
decltype(auto) copy = s.values;
