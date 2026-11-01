// tidal-rules: CE-24
// tidal-expect: run
constinit int counter;
constinit int values[3];
int main() { return counter == 0 && values[1] == 0 ? 0 : 1; }
