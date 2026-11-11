// As a3.cpp, but the copy of the union is read where element 1 lies.
union Row { int items[2]; constexpr Row() {} };
constexpr int f() {
  Row r;
  r.items[0] = 1;
  Row s = r;
  return s.items[1];
}
static_assert(f() == 0);
int main() {}
