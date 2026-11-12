// An indexed d-ary priority queue with a comparator kept by
// [[no_unique_address]], a projection, decrease-key through a position
// table, a heap sort in constant evaluation, and Dijkstra over a small graph.
// No standard header.

using size_t = decltype(sizeof 0);

namespace lib {

template <class T> struct remove_ref { using type = T; };
template <class T> struct remove_ref<T&> { using type = T; };
template <class T> struct remove_ref<T&&> { using type = T; };

template <class T>
constexpr typename remove_ref<T>::type&& move(T&& t) noexcept {
  return static_cast<typename remove_ref<T>::type&&>(t);
}

template <class T>
constexpr void swap(T& a, T& b) noexcept {
  T t = move(a);
  a = move(b);
  b = move(t);
}

struct less {
  template <class A, class B>
    requires requires(const A& a, const B& b) { a < b; }
  constexpr bool operator()(const A& a, const B& b) const { return a < b; }
};

struct greater {
  template <class A, class B>
  constexpr bool operator()(const A& a, const B& b) const { return b < a; }
};

struct identity {
  template <class T>
  constexpr T&& operator()(T&& t) const noexcept { return static_cast<T&&>(t); }
};

template <class T>
concept bool_like = requires(T t) { static_cast<bool>(t); };

template <class C, class A, class B>
concept strict_order = requires(const C& c, const A& a, const B& b) {
  { c(a, b) } -> bool_like;
};

}  // namespace lib

namespace lib {

// D children per node. Items are ids 0..N-1 with a priority each. pos[id]
// is the id's place in the heap, or npos when the id is not queued.
template <class P, size_t N, size_t D = 2, class Cmp = less, class Proj = identity>
  requires (D >= 2)
class IndexedHeap {
 public:
  static constexpr size_t npos = static_cast<size_t>(-1);

  constexpr IndexedHeap() {
    for (size_t i = 0; i < N; ++i) pos_[i] = npos;
  }
  constexpr explicit IndexedHeap(Cmp c, Proj p = {}) : IndexedHeap() {
    cmp_ = c;
    proj_ = p;
  }

  constexpr size_t size() const { return size_; }
  constexpr bool empty() const { return size_ == 0; }
  constexpr bool contains(size_t id) const { return id < N && pos_[id] != npos; }
  constexpr const P& priority(size_t id) const { return prio_[id]; }

  constexpr bool push(size_t id, P p) {
    if (id >= N || contains(id)) return false;
    prio_[id] = move(p);
    heap_[size_] = id;
    pos_[id] = size_;
    up(size_++);
    return true;
  }

  // Moves an id toward the top. Returns false if the new priority is worse.
  constexpr bool improve(size_t id, P p) {
    if (!contains(id) || before(prio_[id], p)) return false;
    prio_[id] = move(p);
    up(pos_[id]);
    return true;
  }

  constexpr size_t top() const { return heap_[0]; }

  constexpr size_t pop() {
    size_t id = heap_[0];
    place(0, heap_[--size_]);
    pos_[id] = npos;
    if (size_ > 0) down(0);
    return id;
  }

 private:
  constexpr bool before(const P& a, const P& b) const { return cmp_(proj_(a), proj_(b)); }
  constexpr bool before_at(size_t i, size_t j) const { return before(prio_[heap_[i]], prio_[heap_[j]]); }
  constexpr void place(size_t i, size_t id) { heap_[i] = id; pos_[id] = i; }
  constexpr void exchange(size_t i, size_t j) {
    size_t a = heap_[i], b = heap_[j];
    place(i, b);
    place(j, a);
  }
  constexpr void up(size_t i) {
    while (i > 0) {
      size_t parent = (i - 1) / D;
      if (!before_at(i, parent)) break;
      exchange(i, parent);
      i = parent;
    }
  }
  constexpr void down(size_t i) {
    for (;;) {
      size_t best = i;
      for (size_t k = 1; k <= D; ++k) {
        size_t c = D * i + k;
        if (c < size_ && before_at(c, best)) best = c;
      }
      if (best == i) return;
      exchange(i, best);
      i = best;
    }
  }

  P prio_[N]{};
  size_t heap_[N]{};
  size_t pos_[N]{};
  size_t size_ = 0;
  [[no_unique_address]] Cmp cmp_{};
  [[no_unique_address]] Proj proj_{};
};

// Sorts in place by pushing every index and popping in order.
template <class T, size_t N, class Cmp = less>
  requires strict_order<Cmp, T, T>
constexpr void heap_sort(T (&a)[N], Cmp cmp = {}) {
  IndexedHeap<T, N, 3, Cmp> h(cmp);
  for (size_t i = 0; i < N; ++i) h.push(i, a[i]);
  T out[N]{};
  for (size_t i = 0; i < N; ++i) out[i] = h.priority(h.pop());
  for (size_t i = 0; i < N; ++i) a[i] = move(out[i]);
}

}  // namespace lib

struct Job {
  int urgency;
  int seq;
  constexpr bool operator<(const Job& o) const {
    return urgency != o.urgency ? urgency > o.urgency : seq < o.seq;
  }
};

struct ByUrgency {
  constexpr int operator()(const Job& j) const { return j.urgency; }
};

struct Edge { int from, to, w; };

template <size_t V, size_t E>
constexpr int shortest(const Edge (&edges)[E], int src, int dst) {
  constexpr int inf = 1 << 29;
  int dist[V];
  for (auto& d : dist) d = inf;
  dist[src] = 0;
  lib::IndexedHeap<int, V> q;
  q.push(static_cast<size_t>(src), 0);
  while (!q.empty()) {
    size_t u = q.pop();
    for (const Edge& e : edges) {
      if (e.from != static_cast<int>(u)) continue;
      int nd = dist[u] + e.w;
      if (nd < dist[e.to]) {
        dist[e.to] = nd;
        size_t t = static_cast<size_t>(e.to);
        if (!q.improve(t, nd)) q.push(t, nd);
      }
    }
  }
  return dist[dst];
}

constexpr Edge graph[] = {
  {0, 1, 7}, {0, 2, 9}, {0, 5, 14}, {1, 2, 10}, {1, 3, 15},
  {2, 3, 11}, {2, 5, 2}, {3, 4, 6}, {5, 4, 9},
};

constexpr bool sorted_ints() {
  int a[] = {5, 3, 9, 1, 4, 1, 8};
  lib::heap_sort(a);
  for (size_t i = 1; i < 7; ++i)
    if (a[i - 1] > a[i]) return false;
  lib::heap_sort(a, lib::greater{});
  if (a[0] != 9 || a[6] != 1) return false;
  int b[] = {31, 12, 25, 44, 3};
  lib::heap_sort(b, [](int x, int y) { return x % 10 < y % 10; });
  return b[0] == 31 && b[1] == 12 && b[2] == 3 && b[4] == 25;
}

struct NoOrder {};
template <class T>
concept sortable_array = requires(T (&a)[2]) { lib::heap_sort(a); };
static_assert(sortable_array<int>);
static_assert(!sortable_array<NoOrder>);

constexpr bool jobs_in_order() {
  lib::IndexedHeap<Job, 4> h;
  h.push(0, Job{1, 0});
  h.push(1, Job{3, 1});
  h.push(2, Job{3, 2});
  h.push(3, Job{2, 3});
  if (h.top() != 1) return false;
  h.improve(0, Job{5, 0});
  size_t order[4];
  for (auto& o : order) o = h.pop();
  return order[0] == 0 && order[1] == 1 && order[2] == 2 && order[3] == 3;
}

constexpr bool projected() {
  lib::IndexedHeap<Job, 3, 4, lib::greater, ByUrgency> h;
  h.push(2, Job{1, 9});
  h.push(0, Job{7, 9});
  h.push(1, Job{4, 9});
  return h.pop() == 0 && h.pop() == 1 && h.pop() == 2 && h.empty();
}

static_assert(sorted_ints());
static_assert(jobs_in_order());
static_assert(projected());
static_assert(shortest<6>(graph, 0, 4) == 20);
static_assert(!lib::IndexedHeap<int, 2>().contains(5));

int main() {
  int r = 0;
  if (!sorted_ints()) r |= 1;
  if (!jobs_in_order()) r |= 2;
  if (!projected()) r |= 4;
  if (shortest<6>(graph, 0, 4) != 20) r |= 8;
  lib::IndexedHeap<int, 3> h;
  if (!h.push(1, 4) || h.push(1, 2) || h.push(7, 1)) r |= 16;
  return r;
}
