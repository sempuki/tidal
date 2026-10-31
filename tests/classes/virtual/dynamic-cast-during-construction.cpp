// tidal-rules: VIRT-10
// tidal-expect: run
struct B;
struct D;
bool cast_null(B* p);
struct B { B() { seen = cast_null(this); } virtual ~B() {} bool seen; };
struct D : B { int v = 0; };
bool cast_null(B* p) { return dynamic_cast<D*>(p) == nullptr; }
int main() { D d; return d.seen ? 0 : 1; }
