// tidal-rules: NAM-35 DER-4
// tidal-expect: accept
struct root { };
struct left : root { };
struct right : root { };
struct join : left, right { void f() { root* p = nullptr; (void)p; } };
