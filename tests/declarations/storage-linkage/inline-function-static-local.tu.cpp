inline int& counter() {
  static int value = 0;
  return value;
}
void bump_other() { ++counter(); }
