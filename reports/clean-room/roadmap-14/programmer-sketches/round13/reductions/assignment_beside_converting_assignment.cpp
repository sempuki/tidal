struct R {
  int* p;
  R& operator=(int v) { *p = v; return *this; }
  R& operator=(const R& r) { *p = *r.p; return *this; }
  operator int() const { return *p; }
};
int main() {
  int a = 0, b = 1;
  R x{&a}, y{&b};
  x = y;
  return a - 1;
}
