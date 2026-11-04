// tidal-rules: VIRT-3 VIRT-7
// tidal-expect: run
// tidal-note: The destructor of a derived class overrides a virtual destructor of its base, so the destructor can carry override and final.
int destroyed = 0;
struct Base { virtual ~Base() { destroyed += 1; } };
struct Middle : Base { ~Middle() override { destroyed += 10; } };
struct Leaf final : Middle { ~Leaf() override final { destroyed += 100; } };
int main() {
  Base* p = new Leaf;
  delete p;
  return destroyed == 111 ? 0 : 1;
}
