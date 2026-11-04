// tidal-rules: LSUP-4 LSUP-7 NEW-33 STG-4 STG-5 STG-6 STG-29 DTOR-6 DED-8 IF-7 IF-9 SW-2 SW-7 TYN-1 TPL-28 SB-1 SB-3 SB-12
// tidal-expect: run
// tidal-note: Write suite W16, program textstats.cpp (reports/clean-room/W16/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// W16 textstats: word, line, and character-class counts over fixed
// string literals, written from the Tidal specification alone. No header
// is included.
//
// Rules used (Tidal IDs):
//   std::size_t                 LSUP-4
//   placement forms             LSUP-7, STG-5, NEW-33
//   raw storage                 STG-4 (unsigned char hosts objects),
//                               STG-P4 and STG-29 (keep the pointer that
//                               placement new returns), STG-6, DTOR-6,
//                               ALN-P1 (alignas on the byte array)
//   if with an initializer      IF-7, IF-9
//   switch over an enumeration  SW-2, SW-7, ENUM-P2
//   type aliases                TYN-1, TPL-28
//   structured bindings         SB-1, SB-3, SB-12, RFOR-P1
//   string literals             PRIM (array of const char, ends in '\0')

namespace std {
using size_t = decltype(sizeof 0);  // LSUP-4
}

// LSUP-7: a program with no header defines the placement forms itself.
inline void* operator new(std::size_t, void* at) noexcept { return at; }
inline void operator delete(void*, void*) noexcept {}

using Count = unsigned long;  // TYN-1
using Text = const char*;

// ------------------------------------------------- character classes

enum class CharClass : unsigned char { Letter, Digit, Space, Newline, Punct, Other };

CharClass classify(char c) {
  if ((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) return CharClass::Letter;
  if (c >= '0' && c <= '9') return CharClass::Digit;
  if (c == '\n') return CharClass::Newline;
  if (c == ' ' || c == '\t') return CharClass::Space;
  switch (c) {
    case '.': case ',': case ';': case ':': case '!': case '?':
    case '\'': case '"': case '-': case '(': case ')':
      return CharClass::Punct;
    default:
      return CharClass::Other;
  }
}

bool is_word_char(char c) {
  CharClass k = classify(c);
  return k == CharClass::Letter || k == CharClass::Digit || c == '\'';
}

char lower(char c) { return (c >= 'A' && c <= 'Z') ? static_cast<char>(c - 'A' + 'a') : c; }

// --------------------------------------------- fixed-capacity container

// FixedVec<T, N> keeps up to N objects of T in a byte array. Each slot
// is a row of bytes. items[i] is the pointer that placement new gave
// for slot i, so the program never casts the bytes back (STG-29).
template <class T, std::size_t N>
class FixedVec {
  alignas(T) unsigned char slots[N][sizeof(T)];
  T* items[N];
  std::size_t count = 0;

public:
  using value_type = T;          // TYN-1 in a class
  using size_type = std::size_t;

  FixedVec() = default;
  FixedVec(const FixedVec&) = delete;  // the pointers point into *this
  FixedVec& operator=(const FixedVec&) = delete;

  ~FixedVec() { clear(); }

  size_type size() const { return count; }
  static constexpr size_type capacity() { return N; }

  // Builds a T in the next slot. Returns null when full.
  template <class... Args>
  T* emplace(Args&&... args) {
    if (count == N) return nullptr;
    T* p = new (slots[count]) T(static_cast<Args&&>(args)...);  // STG-5
    items[count] = p;
    ++count;
    return p;
  }

  void pop() {
    --count;
    items[count]->~T();  // DTOR-6: ends the object; the bytes stay
  }

  void clear() {
    while (count > 0) pop();
  }

  T& operator[](size_type i) { return *items[i]; }
  const T& operator[](size_type i) const { return *items[i]; }

  T* const* begin() const { return items; }
  T* const* end() const { return items + count; }
};

// ---------------------------------------------------- word frequency

int live_entries = 0;  // counts WordEntry objects alive

struct WordEntry {
  Text start;          // first character of the first spelling seen
  std::size_t length;
  Count uses;
  WordEntry(Text s, std::size_t n) : start(s), length(n), uses(1) { ++live_entries; }
  ~WordEntry() { --live_entries; }
  WordEntry(const WordEntry&) = delete;
  WordEntry& operator=(const WordEntry&) = delete;
};

bool same_word(Text a, std::size_t an, Text b, std::size_t bn) {
  if (an != bn) return false;
  for (std::size_t i = 0; i < an; ++i)
    if (lower(a[i]) != lower(b[i])) return false;
  return true;
}

using WordTable = FixedVec<WordEntry, 16>;  // TYN-1, TPL-28-like use

WordEntry* find(WordTable& table, Text s, std::size_t n) {
  for (WordEntry* e : table)
    if (same_word(e->start, e->length, s, n)) return e;
  return nullptr;
}

// ------------------------------------------------------ the counting

struct Stats {
  Count words = 0;
  Count lines = 0;
  Count letters = 0;
  Count digits = 0;
  Count spaces = 0;
  Count punct = 0;
  Count other = 0;
};

// Counts every character class. A switch over the enumeration with one
// case for each enumerator and no default, so SW-7 warns if one is lost.
void tally(Stats& s, CharClass k) {
  switch (k) {
    case CharClass::Letter: ++s.letters; break;
    case CharClass::Digit: ++s.digits; break;
    case CharClass::Space: ++s.spaces; break;
    case CharClass::Newline: ++s.lines; ++s.spaces; break;
    case CharClass::Punct: ++s.punct; break;
    case CharClass::Other: ++s.other; break;
  }
}

struct Result {
  Stats stats;
  Count distinct;
  Count overflowed;  // words that did not fit in the table
};

Result analyze(Text text, WordTable& table) {
  Result r{};
  std::size_t i = 0;
  bool last_was_newline = true;  // an empty text has no lines
  while (text[i] != '\0') {
    char c = text[i];
    tally(r.stats, classify(c));
    last_was_newline = (c == '\n');
    if (is_word_char(c)) {
      std::size_t start = i;
      while (is_word_char(text[i + 1])) {
        ++i;
        tally(r.stats, classify(text[i]));
      }
      std::size_t length = i - start + 1;
      ++r.stats.words;
      // IF-7: the initializer declares e; IF-9: e is visible in both branches.
      if (WordEntry* e = find(table, text + start, length); e != nullptr) {
        ++e->uses;
      } else if (table.emplace(text + start, length) == nullptr) {
        ++r.overflowed;
      }
      last_was_newline = false;
    }
    ++i;
  }
  if (!last_was_newline) ++r.stats.lines;  // a last line with no '\n'
  r.distinct = table.size();
  return r;
}

// The most used word, as a pair of a pointer to the entry and its count.
struct Top {
  const WordEntry* entry;
  Count uses;
};

Top most_used(const WordTable& table) {
  Top best{nullptr, 0};
  for (const WordEntry* e : table)
    if (e->uses > best.uses) best = Top{e, e->uses};
  return best;
}

bool spelled(const WordEntry* e, Text word) {
  std::size_t n = 0;
  while (word[n] != '\0') ++n;
  return e != nullptr && same_word(e->start, e->length, word, n);
}

// ------------------------------------------------- lines and sentences

struct LineInfo {
  Text start;
  std::size_t length;  // without the '\n'
};

using LineTable = FixedVec<LineInfo, 4>;

// Records each line. Returns the number of lines that found the table full.
Count split_lines(Text text, LineTable& table) {
  Count lost = 0;
  std::size_t begin = 0;
  std::size_t i = 0;
  for (;; ++i) {
    char c = text[i];
    if (c != '\n' && c != '\0') continue;
    bool last = (c == '\0');
    if (!(last && i == begin)) {  // no empty line after a final '\n'
      if (table.emplace(text + begin, i - begin) == nullptr) ++lost;
    }
    if (last) break;
    begin = i + 1;
  }
  return lost;
}

struct Longest {
  std::size_t index;
  std::size_t length;
};

Longest longest_line(const LineTable& table) {
  Longest best{0, 0};
  for (std::size_t i = 0; i < table.size(); ++i)
    if (auto [start, length] = table[i]; length > best.length)  // IF-7 + SB-1
      best = Longest{i, length};
  return best;
}

// A sentence ends at '.', '!' or '?' after at least one word character.
enum class SentenceState { Between, InSentence };

Count count_sentences(Text text) {
  Count n = 0;
  SentenceState state = SentenceState::Between;
  for (std::size_t i = 0; text[i] != '\0'; ++i) {
    char c = text[i];
    bool end = (c == '.' || c == '!' || c == '?');
    switch (state) {
      case SentenceState::Between:
        if (is_word_char(c)) state = SentenceState::InSentence;
        break;
      case SentenceState::InSentence:
        if (end) {
          ++n;
          state = SentenceState::Between;
        }
        break;
    }
  }
  return n;
}

// ------------------------------------------------------------ checks

struct Expect {
  Text text;
  Count words, lines, letters, digits, spaces, punct, other;
  Count distinct;
  Count overflowed;  // words that found the table full
  Text top;          // the most used word
  Count top_uses;
  Count sentences;
  Count lines_lost;           // lines that found the line table full
  std::size_t longest;        // length of the longest stored line
};

constexpr Expect samples[] = {
    {"", 0, 0, 0, 0, 0, 0, 0, 0, 0, "", 0, 0, 0, 0},
    {"hello", 1, 1, 5, 0, 0, 0, 0, 1, 0, "hello", 1, 0, 0, 5},
    {"The cat saw the dog.\nThe dog ran!\n", 8, 2, 24, 0, 8, 2, 0, 5, 0, "the", 3, 2, 0, 20},
    {"a1 b22 c333\n\n# done", 4, 3, 7, 6, 5, 0, 1, 4, 0, "a1", 1, 0, 0, 11},
    {"don't stop, don't\tstop", 4, 1, 16, 0, 3, 3, 0, 2, 0, "don't", 2, 0, 0, 22},
    {"Up.\nDown?\nLeft!\nRight.\nBack...\n", 5, 5, 19, 0, 5, 7, 0, 5, 0, "up", 1, 5, 1,
     6},
    {"one two three four five six seven eight nine ten eleven twelve "
     "thirteen fourteen fifteen sixteen seventeen eighteen",
     18, 1, 98, 0, 17, 0, 0, 16, 2, "one", 1, 0, 0, 115},
};

int main() {
  int failures = 0;
  for (const auto& [text, words, lines, letters, digits, spaces, punct, other,
                    distinct, lost, top, top_uses, sentences, lines_lost,
                    longest] : samples) {  // SB-3 over Expect
    {
      WordTable table;
      auto [stats, found, overflowed] = analyze(text, table);  // SB-1
      const auto& [w, l, le, di, sp, pu, ot] = stats;           // SB-3
      if (w != words || l != lines || le != letters || di != digits ||
          sp != spaces || pu != punct || ot != other)
        ++failures;
      if (found != distinct) ++failures;
      if (overflowed != lost) ++failures;
      auto [entry, uses] = most_used(table);
      if (uses != top_uses) ++failures;
      if (uses > 0 && !spelled(entry, top)) ++failures;
      if (live_entries != static_cast<int>(table.size())) ++failures;
    }
    if (live_entries != 0) ++failures;  // the table destroyed every entry

    if (count_sentences(text) != sentences) ++failures;
    LineTable line_table;
    if (split_lines(text, line_table) != lines_lost) ++failures;
    if (line_table.size() + lines_lost != lines) ++failures;
    if (longest_line(line_table).length != longest) ++failures;
  }

  // The container on its own: full means null, and clear ends each object.
  {
    FixedVec<WordEntry, 2> small;
    bool good = small.emplace("ab", 2) != nullptr && small.emplace("cd", 2) != nullptr &&
                small.emplace("ef", 2) == nullptr && live_entries == 2;
    small.clear();
    good = good && live_entries == 0 && small.size() == 0;
    good = good && small.emplace("gh", 2) != nullptr && live_entries == 1;  // reuse
    if (!good) ++failures;
  }
  if (live_entries != 0) ++failures;
  return failures == 0 ? 0 : 1;
}
