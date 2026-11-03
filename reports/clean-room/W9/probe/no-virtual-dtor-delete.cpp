// Probe: an interface without a virtual destructor, deleted through the base (VIRT-12, VIRT-13).
struct P { virtual int run() const = 0; };
struct Q final : P { int run() const override { return 1; } };
int main() { P* p = new Q; int r = p->run(); delete p; return r - 1; }
