// A pure virtual destructor that has a definition, so derived destructors can call it.
inline int log_count = 0;
struct Base { virtual ~Base() = 0; };
Base::~Base() { ++log_count; }
struct Leaf : Base { ~Leaf() override { ++log_count; } };
int main() {
  { Leaf l; }
  Base* b = new Leaf;
  delete b;
  return log_count == 4 ? 0 : 1;
}
