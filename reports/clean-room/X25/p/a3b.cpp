// The same copy as a3.cpp, of a class that is not a union: element 1 of
// items has no value when the implicit copy constructor runs.
struct Row { int items[2]; constexpr Row() {} };
constexpr int f() {
  Row r;
  r.items[0] = 1;
  Row s = r;
  return s.items[0];
}
static_assert(f() == 1);
int main() {}
