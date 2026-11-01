// tidal-rules: STG-23
// tidal-expect: accept
// tidal-note: STG-23 makes the write through a Right undefined, so the test checks only that the program compiles. Only the type sanitizer of Clang 22 (-fsanitize=type) reports the access, and the run goes on with exit status 0.
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
