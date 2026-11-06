// tidal-rules: COND-8 COND-16
// tidal-expect: run
// tidal-note: Decision 0059, item F2 (I4-9). A Derived lvalue against a Base prvalue converts by COND-16 step 3 to a Base prvalue, which is copied from the operand. Base has a deleted move constructor, which the copy does not use.
struct Base {
  int tag = 1;
  Base() = default;
  Base(const Base&) = default;
  Base(Base&&) = delete;
};
struct Derived : Base {
  Derived() { tag = 2; }
};
Base make() { return {}; }
int main(int argc, char**) {
  Derived d;
  const Base& r = argc > 0 ? d : make();
  return r.tag == 2 ? 0 : 1;
}
