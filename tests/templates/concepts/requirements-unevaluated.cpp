// tidal-rules: CPT-30
// tidal-expect: run
// tidal-note: never is declared and not defined, so the program links only if the requirement does not use it. The increment in the requirement does not run.
int never();
int counter = 0;
template <class T> concept Calls = requires { never(); ++counter; };
int main() { return Calls<int> && counter == 0 ? 0 : 1; }
