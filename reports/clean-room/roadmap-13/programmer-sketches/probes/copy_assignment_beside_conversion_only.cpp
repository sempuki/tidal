struct R {
  int* p;
  R& operator=(const R& r) { *p = *r.p; return *this; }
  operator bool() const { return *p != 0; }
};
int main() {
  int a = 0, b = 1;
  R x{&a}, y{&b};
  x = y;
  return a - 1;
}
