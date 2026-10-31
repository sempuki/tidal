// tidal-rules: VIRT-2
// tidal-expect: reject
// tidal-error-clang: virtual. can only appear on non-static member functions
// tidal-error-gcc: cannot be declared both .virtual. and .static
// tidal-error-edg: only nonstatic member functions may be virtual
struct S { virtual static void f(); };
