// tidal-rules: CTOR-4
// tidal-expect: reject
// tidal-error-clang: const type .* without a user-provided default constructor
// tidal-error-gcc: uninitialized .const
// tidal-error-edg: requires an initializer -- class .* has no user-provided default constructor
struct S { S() = default; int v; };
const S s;
