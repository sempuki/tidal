// tidal-rules: SINIT-2
// tidal-expect: run
int counter = 0;
int next() { return ++counter; }
int a = next();
int b = next();
int c = next();
int main() { return a == 1 && b == 2 && c == 3 ? 0 : 1; }
