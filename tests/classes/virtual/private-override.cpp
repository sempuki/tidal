// tidal-rules: VIRT-1
// tidal-expect: run
class B { public: virtual int f() { return 1; } };
class D : public B { int f() override { return 2; } };
int main() { D d; B& r = d; return r.f() == 2 ? 0 : 1; }
