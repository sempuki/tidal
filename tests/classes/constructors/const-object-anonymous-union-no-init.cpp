// tidal-rules: CTOR-24
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: const type .* without a user-provided default constructor
// tidal-error-edg: requires an initializer
// tidal-note: Q-0126. GCC accepts a const object whose anonymous union member has no default member initializer. Clang and EDG reject it.
struct Cell {
  int tag = 1;
  union { int whole; float part; };
};
const Cell cell;
