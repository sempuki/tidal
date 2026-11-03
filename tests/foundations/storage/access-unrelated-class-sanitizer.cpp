// tidal-rules: STG-23
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=type
// tidal-error: with type int \(in Right at offset 0\) accesses an existing object of type int \(in Left at offset 0\)
// tidal-note: Right has the same members as Left, but STG-14 does not list Right for a Left object, so STG-23 makes the write through a Right undefined.
// tidal-note: Run-time check with Clang's type sanitizer. The sanitizer reports the write and lets the program go on, and the runner counts the report as the failure. GCC and Clang 18 have no type sanitizer and skip the test.
struct Left { int value; };
struct Right { int value; };
int store(Left* left, Right* right) {
  left->value = 1;
  right->value = 0;   // undefined: the object is a Left
  return left->value;
}
int main() {
  int (*volatile call)(Left*, Right*) = store;
  Left left{0};
  return call(&left, reinterpret_cast<Right*>(&left));
}
