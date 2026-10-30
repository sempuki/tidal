// tidal-rules: ACC-4
// tidal-expect: accept
class B { protected: static int count; };
int B::count = 0;
class D : public B {
public:
  int get() const { return B::count; }
};
