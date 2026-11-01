// tidal-rules: STG-14
// tidal-expect: run
int through_unsigned(int* number, unsigned* other) { *number = 1; *other = 2; return *number; }
int through_const(int* number, const volatile int* other) { *number = 3; return *other; }
int main() {
  int (*volatile first)(int*, unsigned*) = through_unsigned;
  int (*volatile second)(int*, const volatile int*) = through_const;
  int value = 0;
  int a = first(&value, reinterpret_cast<unsigned*>(&value));
  int b = second(&value, &value);
  return a == 2 && b == 3 ? 0 : 1;
}
