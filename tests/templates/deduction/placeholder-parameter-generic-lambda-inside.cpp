// tidal-rules: DED-34 LAM-15
// tidal-expect: run
// tidal-note: Roadmap round 11, programmer reduction generic_lambda_inside_abbreviated_template.cpp. tally has a parameter of type auto, so it is a function template (DED-34). Inside it a generic lambda captures the parameter, and two calls deduce the lambda's parameter as int and as long (LAM-15).
long tally(auto step) {
  auto add = [step](auto x) { return x + step; };
  return add(1) + add(2L);
}
int main() { return tally(10) == 23 ? 0 : 1; }
