// tidal-rules: CTOR-4
// tidal-expect: reject
// tidal-error-clang: const type .* without a user-provided default constructor
// tidal-error-gcc: uninitialized .const
// tidal-error-edg: requires an initializer -- class .* has no user-provided default constructor
// tidal-note: Gap review 2026-10-31.
struct B { int v; };
struct D : B {};
const D d;
