// tidal-rules: VIRT-10
// tidal-expect: run
struct A { virtual ~A() {} };
struct D : private A { A* self() { return this; } };
int main() { D d; A* a = d.self(); return dynamic_cast<D*>(a) == nullptr ? 0 : 1; }
