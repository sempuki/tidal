template <class Tag> class Hook {
  template <class> friend class IList;
  int v = 4;
};
template <class Tag> class IList {
public:
  int get(Hook<Tag>& h) { return h.v; }
};
struct A {};
int main() { Hook<A> h; IList<A> l; return l.get(h) - 4; }
