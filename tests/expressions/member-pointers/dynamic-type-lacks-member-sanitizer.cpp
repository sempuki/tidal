// tidal-rules: MPTR-6
// tidal-expect: run-fail
// tidal-flags: -O0 -fsanitize=vptr -fno-sanitize-recover=all
// tidal-error: member access within address .* which does not point to an object of type
// tidal-note: Run-time check with the sanitizer (decision 0028). The object is a Base, and the member function belongs to Derived.
struct Base { virtual ~Base() {} int a = 1; };
struct Derived : Base { int b = 2; int get() { return b; } };
int main() {
  int (Base::* call)() = static_cast<int (Base::*)()>(&Derived::get);
  Base* volatile object = new Base;
  int result = (object->*call)();   // undefined: object has no member get
  delete object;
  return result;
}
