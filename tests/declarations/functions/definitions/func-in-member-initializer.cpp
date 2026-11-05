// tidal-rules: FN-5
// tidal-expect: run
// tidal-note: __func__ is declared at the start of the constructor's body, which includes the member initializers. The array has static storage duration, so the pointer stays valid after the constructor returns.
struct Named {
  const char* who;
  Named() : who(__func__) {}
};
int main() { return Named().who[0] == 'N' ? 0 : 1; }
