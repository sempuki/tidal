// tidal-rules: TR-17
// tidal-expect: link-fail
// tidal-sources: main-two-definitions.tu.cpp
// tidal-error-clang: multiple definition of .main
// tidal-error-gcc: multiple definition of .main
// tidal-note: Decision 0059, item RD, with the round 9 writer probe two-mains-a.cpp and two-mains-b.cpp. Each of two units defines main. Each unit compiles, and the link fails because the program has two definitions of main.
int main() { return 0; }
