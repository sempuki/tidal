// tidal-rules: TR-4
// tidal-expect: run
// tidal-sources: units-translated-separately.tu.cpp
int other();
static int local() { return 1; }
int main() { return local() + other() == 3 ? 0 : 1; }
