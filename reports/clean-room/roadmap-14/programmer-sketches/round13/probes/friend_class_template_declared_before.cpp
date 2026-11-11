template <class T, class Tag> class IList;
template <class Tag> class Hook {
  template <class, class> friend class IList;
  int v = 4;
};
template <class T, class Tag> class IList {
public:
  int get(Hook<Tag>& h) { return h.v; }
};
struct A {};
int main() { Hook<A> h; IList<int, A> l; return l.get(h) - 4; }
