// tidal-rules: VIRT-10
// tidal-expect: run
struct A { virtual ~A() {} };
struct D : A {};
int main() {
  A plain;
  A& r = plain;
  try {
    D& d = dynamic_cast<D&>(r);
    (void)d;
  } catch (...) {
    return 0;
  }
  return 1;
}
