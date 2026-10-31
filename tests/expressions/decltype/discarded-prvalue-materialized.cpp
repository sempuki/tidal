// tidal-rules: UNEV-2
// tidal-expect: run
int made = 0, gone = 0;
struct T { T() { ++made; } ~T() { ++gone; } };
int main() { T{}; return made == 1 && gone == 1 ? 0 : 1; }
