// tidal-rules: DTY-3
// tidal-expect: accept
struct S { using type = int; static const int n = 2; };
S s;
decltype(s)::type v = decltype(s)::n;
