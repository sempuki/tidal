// The array member becomes active by an assignment to element 0 (UNI-13),
// so element 1 has no value. The union is then copied by its trivial copy
// constructor during constant evaluation.
union Row { int items[2]; constexpr Row() {} };
constexpr int f() {
  Row r;
  r.items[0] = 1;
  Row s = r;
  return s.items[0];
}
static_assert(f() == 1);
int main() {}
