// tidal-rules: STG-23
// tidal-expect: run
// tidal-expect-gcc-16-o2: run-fail
// tidal-expect-clang-22-o2: run-fail
// tidal-note: At -O2, GCC and Clang assume that a write through a Right does not change a Left and return 1. STG-23 allows any result.
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
