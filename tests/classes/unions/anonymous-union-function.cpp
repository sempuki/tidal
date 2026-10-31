// tidal-rules: UNI-9
// tidal-expect: reject
// tidal-error-clang: functions cannot be declared in an anonymous union
// tidal-error-gcc: anonymous union (may|can) only have
// tidal-error-edg: invalid anonymous union -- member function is not allowed
void f() { union { int i; void g(); }; }
