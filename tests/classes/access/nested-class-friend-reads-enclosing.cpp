// tidal-rules: ACC-10 ACC-5
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-note: Roadmap round 8, programmer reduction nested_friend_reads_enclosing.cpp. Q-0414. A friend function defined in a nested class has the nested class's access, so it reads a private member of the enclosing class. Clang and GCC accept. EDG rejects.
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
