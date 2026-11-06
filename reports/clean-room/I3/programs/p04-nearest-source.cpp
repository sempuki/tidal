// ICS-37 with ICS-42: two conversion functions to Base*; the nearer source wins.
struct Base { int x; };
struct Middle : Base {};
struct Derived : Middle {};
Middle mm;
struct Ptr {
  operator Derived*() const { return nullptr; }
  operator Middle*() const { return &mm; }
};
int main() {
  Base* p = Ptr{};
  return p == &mm ? 0 : 1;
}
