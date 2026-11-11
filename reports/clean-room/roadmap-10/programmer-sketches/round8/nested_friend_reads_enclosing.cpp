class Outer {
  int secret_ = 7;
public:
  struct Inner {
    const Outer* o;
    friend int peek(const Inner& i) { return i.o->secret_; }
  };
  Inner inner() const { return {this}; }
};

int main() {
  Outer x;
  return peek(x.inner()) == 7 ? 0 : 1;
}
