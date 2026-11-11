// tidal-rules: CTOR-10 ICS-9 DED-1
// tidal-expect: run
// tidal-note: Roadmap round 14, programmer probe closure_argument_function_ref_thunk.cpp (from the tries of fnref.cpp). IntFn has a constructor template that is not explicit and takes F&, so a closure lvalue converts to IntFn (CTOR-10, ICS-9). Deduction gives F the closure type (DED-1). The constructor keeps the address of the closure and a function pointer from a lambda with no capture, and that lambda casts the address back to F*. The conversion works for a closure with no capture and for one that captures offset by reference. A later change to offset shows in the last call.
struct IntFn {
  void* target;
  int (*thunk)(void*, int);
  template <class F>
  IntFn(F& f) : target(&f), thunk([](void* t, int v) { return (*static_cast<F*>(t))(v); }) {}
  int operator()(int v) const { return thunk(target, v); }
};
int apply(IntFn fn, int v) { return fn(v); }
int main() {
  auto square = [](int v) { return v * v; };
  if (apply(square, 3) != 9) return 1;
  int offset = 10;
  auto add = [&offset](int v) { return v + offset; };
  if (apply(add, 1) != 11) return 2;
  offset = 20;
  return apply(add, 2) == 22 ? 0 : 3;
}
