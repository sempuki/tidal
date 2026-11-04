// tidal-rules: CTOR-24
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: const type .* without a user-provided default constructor
// tidal-note: Q-0126. No anonymous union gives a member a value, so the const object has no value. Clang rejects. GCC and EDG accept.
struct Cell {
  union { int whole; float part; };
  union { int count; float ratio; };
};
const Cell cell;
