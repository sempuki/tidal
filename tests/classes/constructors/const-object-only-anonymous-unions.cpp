// tidal-rules: CTOR-24
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: const type .* without a user-provided default constructor
// tidal-note: Q-0126. Clang rejects a const object whose data members are all in anonymous unions without default member initializers. GCC and EDG accept it.
struct Cell {
  union { int whole; float part; };
  union { int count; float ratio; };
};
const Cell cell;
