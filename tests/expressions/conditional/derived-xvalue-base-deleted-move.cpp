// tidal-rules: COND-8 COND-16
// tidal-expect: reject
// tidal-error-clang: incompatible operand types
// tidal-error-gcc: use of deleted function
// tidal-error-edg: it is a deleted function
// tidal-note: Decision 0059, item F2 (I4-9). A Derived xvalue against a Base lvalue converts by COND-16 step 3 to a Base prvalue, which is moved from the operand. Base has a deleted move constructor, so the expression is not valid. Clang reports that the operands are incompatible.
template <class T> T&& move_of(T& t) { return static_cast<T&&>(t); }
struct Base {
  Base() = default;
  Base(const Base&) = default;
  Base(Base&&) = delete;
};
struct Derived : Base {};
Derived d;
Base b;
extern bool c;
void f() { (void)(c ? move_of(d) : b); }
