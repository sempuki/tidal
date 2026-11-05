// fixed.h: a small template library shared by every unit.
//
// Rules that decided each construct:
//   include guard ........................................ ODR-P3, INC-1
//   templates and their members defined in the header ..... ODR-P2 (table row "Template")
//   members defined after the class body .................. TPL-23
//   explicit instantiation declaration (extern template) .. INST-P3, INST-27, INST-28
//   inline variable and inline function shared by units ... INL-P1, INL-1, INL-5
//   right shift of a negative value ....................... SHF-P2
//   integer division truncates toward zero ................ MUL-P2
#ifndef TIDAL_W17_FIXED_H
#define TIDAL_W17_FIXED_H

namespace std {
using size_t = decltype(sizeof 0);
}

namespace fx {

// One object for the whole program: counts calls of summarize.
inline int summaries = 0;

// One function for the whole program, with one static local (INL-5).
inline int next_serial() {
  static int last = 0;
  return ++last;
}

// A signed fixed-point number with Frac fraction bits, held in a long long.
template <int Frac>
class Fixed {
public:
  static constexpr long long one = 1LL << Frac;

  constexpr Fixed() : raw_(0) {}
  explicit constexpr Fixed(int whole) : raw_(static_cast<long long>(whole) * one) {}

  static Fixed from_raw(long long raw);
  static Fixed ratio(int numerator, int denominator);

  long long raw() const;
  int floor() const;                        // rounds toward negative infinity
  int round() const;                        // rounds half away from zero

  Fixed operator+(Fixed other) const { return from_raw(raw_ + other.raw_); }
  Fixed operator-(Fixed other) const { return from_raw(raw_ - other.raw_); }
  Fixed operator*(Fixed other) const {
  return from_raw((raw_ * other.raw_) >> Frac);
}
  Fixed operator/(Fixed other) const {
  return from_raw(raw_ * one / other.raw_);
}
  Fixed operator/(int divisor) const { return from_raw(raw_ / divisor); }
  bool operator<(Fixed other) const { return raw_ < other.raw_; }
  bool operator==(const Fixed&) const = default;

  // How many values ratio() has made in this specialization. Not inline:
  // the header defines it as a template member, and the unit with the
  // explicit instantiation definition makes Fixed<16>::made (INST-17).
  static int made;

private:
  long long raw_;
};

template <int Frac>
int Fixed<Frac>::made = 0;

template <int Frac>
Fixed<Frac> Fixed<Frac>::from_raw(long long raw) {
  Fixed f;
  f.raw_ = raw;
  return f;
}

template <int Frac>
Fixed<Frac> Fixed<Frac>::ratio(int numerator, int denominator) {
  ++made;
  return from_raw(static_cast<long long>(numerator) * one / denominator);
}

template <int Frac>
long long Fixed<Frac>::raw() const { return raw_; }

template <int Frac>
int Fixed<Frac>::floor() const { return static_cast<int>(raw_ >> Frac); }

template <int Frac>
int Fixed<Frac>::round() const {
  long long half = one / 2;
  long long r = raw_ < 0 ? -((-raw_ + half) >> Frac) : (raw_ + half) >> Frac;
  return static_cast<int>(r);
}







// The result of summarize.
template <class T>
struct Stats {
  T low;
  T high;
  T total;
  T mean;
  T variance;               // population variance
  int count;

  T range() const;          // high - low
  bool within(T value) const;
};

template <class T>
T Stats<T>::range() const { return high - low; }

template <class T>
bool Stats<T>::within(T value) const { return !(value < low) && !(high < value); }

// Smallest, largest, sum, and mean of count values (count > 0).
// T needs +, -, *, <, and division by int.
template <class T>
Stats<T> summarize(const T* values, int count);

template <class T>
Stats<T> summarize(const T* values, int count) {
  ++summaries;
  Stats<T> s{values[0], values[0], values[0], values[0], T(), count};
  for (int i = 1; i < count; ++i) {
    if (values[i] < s.low) s.low = values[i];
    if (s.high < values[i]) s.high = values[i];
    s.total = s.total + values[i];
  }
  s.mean = s.total / count;
  T squares = T();
  for (int i = 0; i < count; ++i) {
    T d = values[i] - s.mean;
    squares = squares + d * d;
  }
  s.variance = squares / count;
  return s;
}

using Q16 = Fixed<16>;

// Some unit has the definitions of these specializations (INST-27).
extern template class Fixed<16>;
extern template struct Stats<Q16>;
extern template Stats<Q16> summarize<Q16>(const Q16* values, int count);

// The weighted mean of count values. Defined only in the header; the
// unit with the explicit instantiation definition makes the Q16 form.
template <class T>
T weighted_mean(const T* values, const int* weights, int count) {
  T sum = T();
  int total = 0;
  for (int i = 0; i < count; ++i) {
    sum = sum + values[i] * T(weights[i]);
    total += weights[i];
  }
  return sum / total;
}
extern template Q16 weighted_mean<Q16>(const Q16*, const int*, int);

// Shared by every unit, and inline, so each unit defines it.
inline Q16 percent(int p) { return Q16::ratio(p, 100); }

}  // namespace fx

#endif
