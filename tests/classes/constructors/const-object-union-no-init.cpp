// tidal-rules: CTOR-24
// tidal-expect: reject
// tidal-error-clang: const type .* without a user-provided default constructor
// tidal-error-gcc: uninitialized .const
// tidal-error-edg: requires an initializer
union Value { int whole; float part; };
const Value value;
