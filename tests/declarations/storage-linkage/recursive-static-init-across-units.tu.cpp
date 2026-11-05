int& left();
int& right() {
  static int value = left() + 1;
  return value;
}
