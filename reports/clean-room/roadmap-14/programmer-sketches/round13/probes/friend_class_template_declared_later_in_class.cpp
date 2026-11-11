class Hook {
  template <class, class> friend class IList;
  int v = 4;
};
template <class T, class Tag> class IList {
public:
  int get(Hook& h) { return h.v; }
};
int main() { Hook h; IList<int, int> l; return l.get(h) - 4; }
