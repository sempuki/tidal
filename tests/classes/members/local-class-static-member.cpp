// tidal-rules: CLS-28
// tidal-expect: reject
// tidal-error-clang: static data member .count. not allowed in local
// tidal-error-gcc: local class .* shall not have static data member
// tidal-error-edg: static data member declaration is not allowed in this class
void f() {
  struct S { static int count; };
}
