// tidal-rules: TPL-38
// tidal-expect: run
// tidal-note: A friend function template defined in the class, with a default, and declared nowhere else.
struct Host {
  template <class T = int> friend int pick(Host) { return sizeof(T); }
};
int main() { return pick(Host{}) == sizeof(int) ? 0 : 1; }
