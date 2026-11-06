// tidal-rules: CPY-3 OVL-14 DED-1
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction assign_through_constructor_template.cpp. A class has a constructor template that takes any callable and an assignment operator that takes the class by value. Assigning a lambda makes the parameter through the constructor template.
struct Task {
  int result = 0;
  template <class F> Task(F f) : result(f()) {}
  Task& operator=(Task other) {
    result = other.result;
    return *this;
  }
};
int main() {
  Task t([] { return 2; });
  t = [] { return 9; };
  return t.result == 9 ? 0 : 1;
}
