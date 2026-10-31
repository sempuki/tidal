// tidal-rules: CTOR-24
// tidal-expect: run
// tidal-expect-gcc: reject
// tidal-expect-clang-18: reject
// tidal-error-clang-18: const type .* without a user-provided default constructor
// tidal-error-gcc: uninitialized .const
// tidal-note: Q-0126. GCC rejects a const union object without an initializer even when a variant member has a default member initializer. Clang 18 rejects it too. Clang 22 and EDG accept it.
union Value { int whole = 7; float part; };
const Value value;
int main() { return value.whole == 7 ? 0 : 1; }
