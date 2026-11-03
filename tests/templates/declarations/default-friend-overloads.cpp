// tidal-rules: TPL-38 TPL-22
// tidal-expect: run
// tidal-note: Each declaration after the class differs from the friend definition in its function type, its head, or its return type, so each declares another function template. The friend definition stays the only declaration of its template.
struct Host {
  template <class T = int> friend int pick(Host) { return sizeof(T); }
};
template <class T> int pick(Host, int);
template <class T, class U> int pick(Host);
template <class T> long pick(Host);
template <int N> int pick(Host);
int main() { return pick(Host{}) == sizeof(int) ? 0 : 1; }
