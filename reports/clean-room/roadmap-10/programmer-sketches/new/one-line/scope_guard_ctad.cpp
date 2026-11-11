// A scope guard that runs a lambda at the end of its scope, its type deduced by CTAD.
template <class F> struct Guard {
  F f; bool armed = true;
  Guard(F fn) : f(fn) {}
  ~Guard() { if (armed) f(); }
};
int main() {
  int closed = 0;
  {
    Guard g([&closed] { ++closed; });
    Guard h([&closed] { closed += 10; });
    h.armed = false;
  }
  return closed == 1 ? 0 : 1;
}
