// tidal-rules: DED-34 LAM-15
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer reduction generic_lambda_returned_from_abbreviated_template.cpp. offset has a parameter of type auto, so it is a function template (DED-34). It returns a generic lambda that captures the parameter, and the call of the result deduces the lambda's own parameter (LAM-15). The program behaves as with template <class B> offset(B base).
auto offset(auto base) {
  return [base](auto x) { return x + base; };
}
int main() {
  auto by_five = offset(5);
  return (by_five(4) == 9 && offset(1.5)(2) == 3.5) ? 0 : 1;
}
