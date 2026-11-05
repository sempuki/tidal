// tidal-rules: SINIT-7 SINIT-2
// tidal-expect: run
// tidal-note: Holder::middle is an inline static data member defined in its class. It takes its place at the class definition, between the two inline variables.
char trace[3];
int pos = 0;
int mark(char c) { trace[pos++] = c; return 0; }
inline int first = mark('1');
struct Holder { static inline int middle = mark('2'); };
inline int last = mark('3');
int main() { return trace[0] == '1' && trace[1] == '2' && trace[2] == '3' ? 0 : 1; }
