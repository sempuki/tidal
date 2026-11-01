// tidal-rules: SINIT-7
// tidal-expect: run
int counter = 0;
int next() { return ++counter; }
struct Registry { static int id; };
int a = next();
int Registry::id = next();
int c = next();
int main() { return a == 1 && Registry::id == 2 && c == 3 ? 0 : 1; }
