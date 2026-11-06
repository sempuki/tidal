// OVL-26 (Q-0189): the constructor's result beats a conversion to a derived class.
struct Source;
struct Target {
  int from;
  Target(const Source&) : from(1) {}
  Target(int v) : from(v) {}
};
struct Derived : Target { Derived() : Target(2) {} };
struct Source { operator Derived() const { return Derived(); } };
int main() {
  const Source s;
  Target t = s;
  return t.from == 1 ? 0 : 1;
}
