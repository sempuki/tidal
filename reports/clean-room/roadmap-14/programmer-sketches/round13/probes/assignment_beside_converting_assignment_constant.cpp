struct R {
  int* p;
  constexpr R& operator=(bool v) { *p = v; return *this; }
  constexpr R& operator=(const R& r) { return *this = static_cast<bool>(r); }
  constexpr operator bool() const { return *p != 0; }
};
constexpr int f() {
  int a = 0, b = 1;
  R x{&a}, y{&b};
  x = y;
  return a;
}
static_assert(f() == 1);
int main() { return f() - 1; }
