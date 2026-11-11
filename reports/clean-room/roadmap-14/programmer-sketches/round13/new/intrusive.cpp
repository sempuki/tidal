// IList<T, Tag>: an intrusive doubly linked list. An element derives from
// Hook<Tag> once for each list it can be in, so one object sits in two
// lists at once. The list owns no element: a hook unlinks itself when its
// object ends, and a copy of an object is not linked. Iterators, insert
// in order by a comparison, erase, splice, and constant tests.
namespace std {
using size_t = decltype(sizeof 0);
template <class E> class initializer_list {
  const E* b_;
  size_t n_;
  constexpr initializer_list(const E* b, size_t n) : b_(b), n_(n) {}

public:
  constexpr initializer_list() : b_(nullptr), n_(0) {}
  constexpr const E* begin() const { return b_; }
  constexpr const E* end() const { return b_ + n_; }
  constexpr size_t size() const { return n_; }
};
}

template <class T, class Tag> class IList;

template <class Tag>
class Hook {
  template <class, class> friend class IList;
  Hook* prev_ = nullptr;
  Hook* next_ = nullptr;

  constexpr void link_before(Hook* at) {
    prev_ = at->prev_;
    next_ = at;
    at->prev_->next_ = this;
    at->prev_ = this;
  }

public:
  constexpr Hook() = default;
  constexpr Hook(const Hook&) {}
  constexpr Hook& operator=(const Hook&) { return *this; }
  constexpr ~Hook() { unlink(); }
  constexpr bool linked() const { return next_ != nullptr; }
  constexpr void unlink() {
    if (!next_) return;
    prev_->next_ = next_;
    next_->prev_ = prev_;
    prev_ = next_ = nullptr;
  }
};

template <class T, class Tag>
class IList {
  using H = Hook<Tag>;
  H head_;

  static constexpr T& owner(H* h) { return static_cast<T&>(*h); }

public:
  constexpr IList() { head_.prev_ = head_.next_ = &head_; }
  IList(const IList&) = delete;
  IList& operator=(const IList&) = delete;
  constexpr ~IList() { clear(); }

  class iterator {
    friend class IList;
    H* h_;
    constexpr explicit iterator(H* h) : h_(h) {}

  public:
    constexpr T& operator*() const { return owner(h_); }
    constexpr T* operator->() const { return &owner(h_); }
    constexpr iterator& operator++() { h_ = h_->next_; return *this; }
    constexpr iterator& operator--() { h_ = h_->prev_; return *this; }
    constexpr bool operator==(const iterator&) const = default;
  };

  constexpr iterator begin() { return iterator(head_.next_); }
  constexpr iterator end() { return iterator(&head_); }
  constexpr bool empty() const { return head_.next_ == &head_; }
  constexpr std::size_t size() const {
    std::size_t n = 0;
    for (const H* h = head_.next_; h != &head_; h = h->next_) ++n;
    return n;
  }
  constexpr T& front() { return owner(head_.next_); }
  constexpr T& back() { return owner(head_.prev_); }

  constexpr void push_back(T& x) { static_cast<H&>(x).unlink(); static_cast<H&>(x).link_before(&head_); }
  constexpr void push_front(T& x) { static_cast<H&>(x).unlink(); static_cast<H&>(x).link_before(head_.next_); }
  constexpr iterator insert(iterator at, T& x) {
    H& h = x;
    h.unlink();
    h.link_before(at.h_);
    return iterator(&h);
  }
  template <class Less>
  constexpr iterator insert_sorted(T& x, Less less) {
    iterator it = begin();
    while (it != end() && !less(x, *it)) ++it;
    return insert(it, x);
  }
  constexpr iterator erase(iterator it) {
    iterator next(it.h_->next_);
    it.h_->unlink();
    return next;
  }
  constexpr void pop_front() { erase(begin()); }
  constexpr void clear() {
    while (!empty()) pop_front();
  }
  // Moves every element of other to the end of this list.
  constexpr void splice(IList& other) {
    while (!other.empty()) push_back(other.front());
  }
};

struct ByAge {};
struct ByName {};

struct Person : Hook<ByAge>, Hook<ByName> {
  const char* name;
  int age;
  constexpr Person(const char* n, int a) : name(n), age(a) {}
};

constexpr bool before(const char* a, const char* b) {
  for (; *a && *a == *b; ++a, ++b) {}
  return *a < *b;
}

constexpr int ages(IList<Person, ByAge>& l) {
  int r = 0;
  for (Person& p : l) r = r * 100 + p.age;
  return r;
}

constexpr bool constant_test() {
  IList<Person, ByAge> by_age;
  IList<Person, ByName> by_name;
  Person a("cy", 30), b("al", 20), c("bo", 40);
  for (Person* p : {&a, &b, &c}) {
    by_age.insert_sorted(*p, [](const Person& x, const Person& y) { return x.age < y.age; });
    by_name.insert_sorted(*p, [](const Person& x, const Person& y) { return before(x.name, y.name); });
  }
  if (ages(by_age) != 203040) return false;
  if (by_name.front().age != 20 || by_name.back().age != 30) return false;
  {
    Person d("ad", 10);
    by_age.push_front(d);
    if (by_age.size() != 4 || !d.Hook<ByAge>::linked() || d.Hook<ByName>::linked()) return false;
  }
  if (by_age.size() != 3) return false;
  Person copy = a;
  if (copy.Hook<ByAge>::linked()) return false;
  by_age.erase(by_age.begin());
  return ages(by_age) == 3040 && by_name.size() == 3;
}
static_assert(constant_test());

int main() {
  IList<Person, ByAge> young, old;
  Person p[] = {{"ann", 31}, {"ben", 12}, {"cat", 67}, {"dan", 45}, {"eve", 8}};
  for (Person& x : p) (x.age < 40 ? young : old).push_back(x);
  if (young.size() != 3 || old.size() != 2) return 1;
  young.splice(old);
  if (!old.empty() || young.size() != 5) return 2;
  int sum = 0;
  for (auto it = young.begin(); it != young.end(); ++it) sum += it->age;
  if (sum != 163) return 3;
  auto last = young.end();
  --last;
  if (last->age != 45) return 4;
  p[2].Hook<ByAge>::unlink();
  if (young.size() != 4) return 5;
  return 0;
}
