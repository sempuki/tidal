// tidal-rules: STG-23
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=type
// tidal-error: write of size 8 .* with type E accesses an existing object of type long
// tidal-note: The object is a long. STG-14 does not list the enumeration E for it, although E has long as its underlying type, so STG-23 makes the write undefined.
// tidal-note: Run-time check with Clang's type sanitizer. GCC and Clang 18 have no type sanitizer and skip the test. At -O2, Clang 22 and GCC 16 assume that the write through E does not change the long, and the function returns 1. At -O0 it returns 2.
enum class E : long { };
long store(long* number, E* tag) {
  *number = 1;
  *tag = E(2);   // undefined: the object is a long
  return *number;
}
int main() {
  long (*volatile call)(long*, E*) = store;
  long value = 0;
  return static_cast<int>(call(&value, reinterpret_cast<E*>(&value)));
}
