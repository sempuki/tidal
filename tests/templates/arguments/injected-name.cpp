// tidal-rules: TARG-11
// tidal-expect: accept
// tidal-note: Inside the template, Node alone names the current specialization, and Node<U> names another one.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
template <class T> struct Node {
  using self = Node;
  using other = Node<char>;
  Node* next = nullptr;
};
static_assert(same<Node<int>::self, Node<int>>);
static_assert(same<Node<int>::other, Node<char>>);
