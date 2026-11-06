// OVL-29 before OVL-27: the template that returns T&& serves F&&.
using F = int();
int a() { return 1; }
int b() { return 2; }
struct Source {
  operator F&() const { return a; }
  template <class T> operator T&&() const { return b; }
};
int main() {
  F&& r = Source{};
  return r() == 2 ? 0 : 1;
}
