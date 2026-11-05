// tidal-rules: LSUP-7 LSUP-5 STG-5 NEW-19 LAM-1 CPT-1 CPT-24 RFOR-1 DTOR-6
// tidal-expect: run
// tidal-note: Roadmap round 4, programmer sketch smallvec_nongeneric.cpp (reports/clean-room/roadmap-4/programmer.md, section 1.5). The rules are the ones the report names for this program and its reductions, and the rules of the main constructs it uses.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// A generic growable vector with a constrained algorithm and a generic lambda.
using size_t = decltype(sizeof 0);
namespace std { enum class align_val_t : size_t {}; }
inline void* operator new(size_t, void* p) noexcept { return p; }
template <class T> concept Movable = requires(T a, T b) { T(static_cast<T&&>(a)); };
template <Movable T> class Vec {
  T* data_ = nullptr; size_t size_ = 0, cap_ = 0;
  void grow() {
    size_t nc = cap_ ? cap_ * 2 : 2;
    T* nd = static_cast<T*>(::operator new(nc * sizeof(T)));
    for (size_t i = 0; i < size_; ++i) { ::new (static_cast<void*>(nd + i)) T(static_cast<T&&>(data_[i])); data_[i].~T(); }
    ::operator delete(data_); data_ = nd; cap_ = nc;
  }
public:
  Vec() = default;
  Vec(const Vec&) = delete;
  ~Vec() { for (size_t i = 0; i < size_; ++i) data_[i].~T(); ::operator delete(data_); }
  void push(T v) { if (size_ == cap_) grow(); ::new (static_cast<void*>(data_ + size_)) T(static_cast<T&&>(v)); ++size_; }
  T* begin() { return data_; } T* end() { return data_ + size_; }
  size_t size() const { return size_; }
  T& operator[](size_t i) { return data_[i]; }
};
template <class T> void swap_(T& a, T& b) { T t(static_cast<T&&>(a)); a = static_cast<T&&>(b); b = static_cast<T&&>(t); }
template <class It, class Less> void sort(It b, It e, Less less) {
  for (It i = b; i != e; ++i) for (It j = i + 1; j != e; ++j)
    if (less(*j, *i)) swap_(*i, *j);
}
struct Name { const char* s; int n; };
int main() {
  Vec<Name> v;
  int vals[] = {5, 3, 9, 1, 7};
  for (int x : vals) v.push(Name{"x", x});
  sort(v.begin(), v.end(), [](const Name& a, const Name& b) { return a.n < b.n; });
  int prev = -1;
  for (auto& e : v) { if (e.n < prev) return 1; prev = e.n; }
  return v.size() == 5 ? 0 : 2;
}
