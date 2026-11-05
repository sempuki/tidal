// tidal-rules: LAM-1 LAM-15
// tidal-expect: run
// tidal-note: Roadmap round 4, programmer sketch lambda_generic.cpp (reports/clean-room/roadmap-4/programmer.md, section 1.5). The rules are the ones the report names for this program and its reductions, and the rules of the main constructs it uses.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
int main() { auto f = [](auto a, auto b) { return a < b; }; return f(1, 2) && !f(2.0, 1.0) ? 0 : 1; }
