// tidal-rules: ACC-20
// tidal-expect: run
// tidal-note: Roadmap round 13, programmer reduction friend_class_template_declared_later_in_class_template.cpp (from intrusive_first_try.cpp). The friend declaration with a template head names the class template Chain before any other declaration of Chain. Every specialization of Chain is a friend of every specialization of Link (ACC-20).
template <class Tag> class Link {
  template <class> friend class Chain;
  int weight = 6;
};
template <class Tag> class Chain {
public:
  int read(const Link<Tag>& l) const { return l.weight; }
};
struct Red {};
int main() {
  Link<Red> l;
  Chain<Red> c;
  return c.read(l) == 6 ? 0 : 1;
}
