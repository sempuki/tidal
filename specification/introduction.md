# 0. Provenance

Tidal is made with deep respect for copyright, and for the years of
careful work that the many contributors to the ISO C++ standard have
given it.

**Tidal holds no text of the ISO standard, and rejects text that is
similar to it.** On every change, a tool compares the pages of the Tidal
specification with the text of the ISO standard, and a similar passage
fails the change. The same tool checks each summary that a source
reader writes, before a clean-room writer reads it.

## Definitions

| Term | Meaning |
|---|---|
| **ISO standard** | ISO/IEC 14882:2024, the international standard that defines C++23, with its drafts. Tidal does not copy it, and the agents that write Tidal do not read it. |
| **Tidal specification** | The pages of this site: chapters 1 to 8, the glossary, and the rule index. Their source is the directory `specification/` of the Tidal repository. |
| **Baseline compilers** | Clang, GCC, and EDG, the compilers that {ref}`the front page <baseline-compilers>` lists. Tidal runs them as programs and records what they do. |
| **Tidal front ends** | The programs in the directory `frontend/` of the Tidal repository. Each one reads a C++ program and decides what the Tidal specification says about it. `frontend/next/` holds the newer of the two. |

The Tidal specification describes the same language as the ISO
standard, so that a program means the same under both. The ISO
standard remains the normative definition of C++. Tidal does not
replace it.

## Roles

| Agent | Forbidden | Reads | Writes |
|---|---|---|---|
| **Source reader** | Writing or changing any file of the repository; quoting a source or proposing wording | The ISO standard, WG21 papers, cppreference, compiler sources under restrictive licenses, and posts that quote any of these | Summaries in its own words, which describe behavior, gaps, and doubts |
| **Clean-room writer** | Reading protected material | Source readers' summaries that passed the non-similarity check, the behavior and diagnostics of the baseline compilers, sanitized compiler sources under permissive licenses, and every file of the repository | The Tidal specification, the tests, the tools, and the documentation |
| **Clean-room reader** | Reading anything other than Tidal's own files | The Tidal specification, the tests, and their results | Its own exercise directories, and the Tidal front ends |

An agent that reads protected material becomes a source reader for the
rest of its session. It cannot work in the clean room again in that
session.

A source reader's summary describes language behavior, gaps, and
doubts. It contains no code, no quoted or paraphrased text, no proposed
wording, and no section numbers from a source. Before a clean-room
writer reads a summary, the tool `tools/similarity.py` compares the
summary with a reference corpus. A summary with a flagged passage goes
back to the source reader to be reworded.

## Workflow

```{raw} html
<figure class="tidal-diagram" tabindex="0" role="button" aria-label="The Tidal workflow. Select to enlarge." title="Click to enlarge" onclick="this.classList.toggle('zoomed')" onkeydown="if (event.key === 'Enter' || event.key === ' ') { event.preventDefault(); this.classList.toggle('zoomed'); } else if (event.key === 'Escape') { this.classList.remove('zoomed'); }">
<svg font-family="sans-serif" viewBox="0 0 890 500" role="img" aria-labelledby="wf-title wf-desc">
  <title id="wf-title">The Tidal workflow</title>
  <desc id="wf-desc">The clean room is the shaded area on the left. The numbers are the steps of the pipeline listed below the diagram. 1: a clean-room writer takes the topic from the status page. 2: the writer probes the baseline compilers for their behavior and diagnostics. 3: the writer reads the source readers' summaries that passed the non-similarity check. Outside the clean room, source readers read the ISO standard and other protected sources. 4, 5, and 6: the writer classifies the behavior and writes the rules with their kinds, the tests, and the questions. 7: the non-similarity check compares the page with the reference corpus. 8: the tests run on the compilers, and the results become each page's Evidence. Clean-room readers, including the Tidal front ends, read only Tidal's files and send findings back to the writers.</desc>
  <defs>
    <marker id="wf-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
      <path d="M0,0 L10,5 L0,10 z" class="wf-head" fill="currentColor"/>
    </marker>
  </defs>
  <!-- the clean room -->
  <rect x="4" y="4" width="632" height="492" rx="10" class="wf-room" fill="#2962ff" fill-opacity="0.06"/>
  <text x="20" y="28" class="wf-note" fill="currentColor" font-size="12" text-anchor="start" font-weight="bold">Clean room</text>
  <text x="656" y="28" class="wf-note" fill="currentColor" font-size="12" text-anchor="start" font-weight="bold">Outside the clean room</text>

  <!-- boxes -->
  <rect x="250" y="40" width="200" height="50" rx="6" class="wf-box" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="350.0" y="61.0" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Compilers</text>
  <text x="350.0" y="79.0" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">Clang, GCC, EDG</text>
  <rect x="250" y="200" width="200" height="50" rx="6" class="wf-box wf-accent" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="350.0" y="230.0" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Clean-room writers</text>
  <rect x="250" y="370" width="200" height="56" rx="6" class="wf-box wf-accent" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="350.0" y="394.0" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Tidal specification</text>
  <text x="350.0" y="412.0" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">and tests</text>
  <rect x="30" y="370" width="170" height="56" rx="6" class="wf-box" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="115.0" y="394.0" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Clean-room readers</text>
  <text x="115.0" y="412.0" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">and front ends</text>
  <rect x="470" y="200" width="140" height="50" rx="6" class="wf-box" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="540.0" y="221.0" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Non-similarity</text>
  <text x="540.0" y="239.0" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">check</text>
  <rect x="680" y="200" width="190" height="50" rx="6" class="wf-box" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="775.0" y="230.0" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Source readers</text>
  <rect x="680" y="50" width="190" height="56" rx="6" class="wf-box wf-protected" fill="none" stroke="currentColor" stroke-width="1.2" stroke-dasharray="5 4"/>
  <text x="775.0" y="74.0" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Protected sources</text>
  <text x="775.0" y="92.0" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">the ISO standard and others</text>

  <!-- step 1: the topic comes from the status page -->
  <rect x="40" y="112" width="150" height="40" rx="6" class="wf-box" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="115" y="137" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Status page</text>
  <path d="M190,132 L285,132 L285,198" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <rect x="198" y="114" width="18" height="18" rx="9" class="wf-badge" fill="currentColor"/>
  <text x="207" y="127" class="wf-badge-text" fill="white" font-size="11" font-weight="bold" text-anchor="middle">1</text>
  <text x="221" y="127" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">topic</text>

  <!-- step 2 -->
  <line x1="350" y1="90" x2="350" y2="198" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <rect x="359.0" y="131" width="18" height="18" rx="9" class="wf-badge" fill="currentColor"/>
  <text x="368" y="144" class="wf-badge-text" fill="white" font-size="11" font-weight="bold" text-anchor="middle">2</text>
  <text x="382" y="144" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">behavior and</text>
  <text x="382" y="160" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">diagnostics</text>

  <!-- steps 4, 5, and 6: classify, draft, questions -->
  <line x1="350" y1="250" x2="350" y2="368" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <rect x="349" y="296" width="50" height="18" rx="9" class="wf-badge" fill="currentColor"/>
  <text x="374" y="309" class="wf-badge-text" fill="white" font-size="11" font-weight="bold" text-anchor="middle">4, 5, 6</text>
  <text x="405" y="309" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">rules and their kinds,</text>
  <text x="405" y="325" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">tests, questions</text>

  <!-- step 7 -->
  <path d="M450,398 L540,398 L540,252" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <rect x="549.0" y="321" width="18" height="18" rx="9" class="wf-badge" fill="currentColor"/>
  <text x="558" y="334" class="wf-badge-text" fill="white" font-size="11" font-weight="bold" text-anchor="middle">7</text>
  <text x="572" y="334" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">pages</text>

  <!-- step 8 -->
  <path d="M350,426 L350,456 L18,456 L18,65 L248,65" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <rect x="25.0" y="469" width="18" height="18" rx="9" class="wf-badge" fill="currentColor"/>
  <text x="34" y="482" class="wf-badge-text" fill="white" font-size="11" font-weight="bold" text-anchor="middle">8</text>
  <text x="50" y="482" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">tests run on the compilers; results go to each page's Evidence</text>

  <!-- findings -->
  <line x1="250" y1="398" x2="202" y2="398" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <path d="M115,370 L115,225 L248,225" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <text x="125" y="290" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">findings: gaps,</text>
  <text x="125" y="306" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">doubts, failures</text>

  <!-- step 3 -->
  <line x1="775" y1="106" x2="775" y2="198" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <text x="783" y="156" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">read</text>
  <line x1="680" y1="225" x2="612" y2="225" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <text x="646" y="213" class="wf-edge" fill="currentColor" font-size="12" text-anchor="middle">summaries</text>
  <line x1="470" y1="225" x2="452" y2="225" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <rect x="461.0" y="265" width="18" height="18" rx="9" class="wf-badge" fill="currentColor"/>
  <text x="470" y="278" class="wf-badge-text" fill="white" font-size="11" font-weight="bold" text-anchor="middle">3</text>
  <text x="484" y="278" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">passed</text>
</svg>
<figcaption>The numbers are the steps of the pipeline in the list
below. One badge can mark more than one step. The findings of
clean-room readers are not a step of the pipeline. They come back to the
writers at any time. Everything inside the clean room comes from the
baseline compilers, from summaries that passed the non-similarity check,
and from Tidal's own files.</figcaption>
</figure>
```

A page of the Tidal specification goes through these steps
(`documentation/pipeline.md` in the repository gives each one in
full):

1. **Topic.** A clean-room writer takes the next step of the status
   page (`documentation/status.md`) and picks the language feature that
   the page will cover.
2. **Probe.** The writer runs small programs on the baseline compilers
   and records what each compiler accepts, rejects, and prints.
3. **Read sources.** The writer reads the sanitized, permissively
   licensed compiler sources where they help, and the source readers'
   summaries that passed the non-similarity check.
4. **Classify.** The writer sorts the behavior into kinds: required,
   rejected, undefined, unspecified, implementation-defined.
5. **Draft.** The writer writes the page's principles and rules. Each
   rule has an ID, a kind, and test programs that show it.
6. **Questions.** Where the compilers disagree, the writer adopts a
   provisional answer, records it as a question, and goes on.
7. **Non-similarity check.** `tools/similarity.py` compares the page with
   the reference corpus, and the writer rewrites each flagged passage.
8. **Evidence.** Every test runs on every baseline compiler, and the
   results go into the Evidence segment of the page.

The repository's history records each step. Each step is one commit,
pushed when it is made, and the history is never rewritten.

## Evolution

The Tidal specification improves through checks that never use the
text of the ISO standard. Each check finds a fault in a specific rule.

**The baseline compilers decide the behavior.** A rule of the Tidal
specification states what the baseline compilers do. Where they agree,
the rule records the agreement, and its tests show it. Where they
disagree, a question in the repository records the split and the answer
that Tidal adopts.

**The tests tie every rule to programs.** Each rule cites tests, and
each test states its expected outcome: accepted, rejected with a named
rule, run to a result, or stopped by undefined behavior. When a
compiler's result differs from the expected outcome, the rule or the
test is wrong.

**Clean-room readers use the text as its audience would.** A
clean-room reader writes programs from the Tidal specification alone,
predicts the outcomes of tests, or writes a guide to a task. A wrong
prediction, or a program that the reader cannot write, points to a rule
that is missing or unclear. The finding goes to the clean-room writers,
who change the wording.

**The Tidal front ends test the rules as code.** Clean-room readers
write the Tidal front ends from the Tidal specification and the Tidal
tests. They do not read the ISO standard or any compiler source. Each
rule of the Tidal specification becomes code that decides real
programs, and the front end runs on every test. Where a front end
cannot decide a program, or decides it against a test's expected
outcome, either the code is wrong or the rule is unclear or incomplete.
Vague wording cannot pass this check. The front end also lets a writer
try a change of wording against the whole test suite at once.

**Source readers add coverage, not text.** A source reader can report
that the Tidal specification misses a case, or that a rule is too
broad. The report passes the non-similarity check, and a clean-room writer
then probes the compilers and writes the rule independently.

**Closeness to the ISO standard is measured.** Tidal is also an
experiment in how close independent text ends up to the wording of the
ISO standard. The non-similarity checks that guard each step also measure
that closeness.
