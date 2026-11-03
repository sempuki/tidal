namespace std {
// The three ordering classes of LSUP-2. Each holds -1, 0, 1, or (for
// partial_ordering) 2 for unordered. The comparisons with a literal 0
// are what the rewritten candidates of OVL-13 call.
struct partial_ordering {
  int v;
  static const partial_ordering less, equivalent, greater, unordered;
  friend constexpr bool operator==(partial_ordering, partial_ordering) = default;
  friend constexpr bool operator==(partial_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(partial_ordering a, int) { return a.v == -1; }
  friend constexpr bool operator>(partial_ordering a, int) { return a.v == 1; }
  friend constexpr bool operator<=(partial_ordering a, int) { return a.v == -1 || a.v == 0; }
  friend constexpr bool operator>=(partial_ordering a, int) { return a.v == 1 || a.v == 0; }
  friend constexpr bool operator<(int, partial_ordering a) { return a.v == 1; }
  friend constexpr bool operator>(int, partial_ordering a) { return a.v == -1; }
  friend constexpr bool operator<=(int, partial_ordering a) { return a.v == 1 || a.v == 0; }
  friend constexpr bool operator>=(int, partial_ordering a) { return a.v == -1 || a.v == 0; }
};
inline constexpr partial_ordering partial_ordering::less{-1};
inline constexpr partial_ordering partial_ordering::equivalent{0};
inline constexpr partial_ordering partial_ordering::greater{1};
inline constexpr partial_ordering partial_ordering::unordered{2};

struct weak_ordering {
  int v;
  static const weak_ordering less, equivalent, greater;
  constexpr operator partial_ordering() const { return partial_ordering{v}; }
  friend constexpr bool operator==(weak_ordering, weak_ordering) = default;
  friend constexpr bool operator==(weak_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(weak_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(weak_ordering a, int) { return a.v > 0; }
  friend constexpr bool operator<=(weak_ordering a, int) { return a.v <= 0; }
  friend constexpr bool operator>=(weak_ordering a, int) { return a.v >= 0; }
  friend constexpr bool operator<(int, weak_ordering a) { return 0 < a.v; }
  friend constexpr bool operator>(int, weak_ordering a) { return 0 > a.v; }
  friend constexpr bool operator<=(int, weak_ordering a) { return 0 <= a.v; }
  friend constexpr bool operator>=(int, weak_ordering a) { return 0 >= a.v; }
};
inline constexpr weak_ordering weak_ordering::less{-1};
inline constexpr weak_ordering weak_ordering::equivalent{0};
inline constexpr weak_ordering weak_ordering::greater{1};

struct strong_ordering {
  int v;
  static const strong_ordering less, equal, equivalent, greater;
  constexpr operator partial_ordering() const { return partial_ordering{v}; }
  constexpr operator weak_ordering() const { return weak_ordering{v}; }
  friend constexpr bool operator==(strong_ordering, strong_ordering) = default;
  friend constexpr bool operator==(strong_ordering a, int) { return a.v == 0; }
  friend constexpr bool operator<(strong_ordering a, int) { return a.v < 0; }
  friend constexpr bool operator>(strong_ordering a, int) { return a.v > 0; }
  friend constexpr bool operator<=(strong_ordering a, int) { return a.v <= 0; }
  friend constexpr bool operator>=(strong_ordering a, int) { return a.v >= 0; }
  friend constexpr bool operator<(int, strong_ordering a) { return 0 < a.v; }
  friend constexpr bool operator>(int, strong_ordering a) { return 0 > a.v; }
  friend constexpr bool operator<=(int, strong_ordering a) { return 0 <= a.v; }
  friend constexpr bool operator>=(int, strong_ordering a) { return 0 >= a.v; }
};
inline constexpr strong_ordering strong_ordering::less{-1};
inline constexpr strong_ordering strong_ordering::equal{0};
inline constexpr strong_ordering strong_ordering::equivalent{0};
inline constexpr strong_ordering strong_ordering::greater{1};
}
