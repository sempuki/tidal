// tidal-rules: CPY-18
// tidal-expect: run
// tidal-note: The implicit move assignment operator, with a virtual base reached through two paths. One or two assignments of the virtual base are allowed.
int assigns = 0;
struct V {
  V() = default;
  V& operator=(V&&) { ++assigns; return *this; }
};
struct A : virtual V {};
struct B : virtual V {};
struct D : A, B {};
int main() {
  D x, y;
  x = static_cast<D&&>(y);
  return (assigns == 1 || assigns == 2) ? 0 : 1;
}
