# 0. Provenance

Tidal is made with deep respect for copyright, and for the years of
careful work that the many contributors to the ISO C++ standard have
given it.

This chapter is the statement of provenance of the Tidal
specification. It says who writes the text, what each writer may read,
and how the text improves without the text of the ISO standard. It
states no language rules. Chapters 1 to 8 state the rules.

## Names used in this chapter

This chapter names two specifications and two kinds of program. Each
name below means one thing only.

| Name | What it is |
|---|---|
| **The ISO standard** | ISO/IEC 14882:2024, the international standard that defines C++23, with its drafts. Tidal does not copy it, and the agents that write Tidal do not read it. |
| **The Tidal specification** | The pages of this site: chapters 1 to 8, the glossary, and the rule index. Their source is the directory `specification/` of the Tidal repository. |
| **The baseline compilers** | Clang, GCC, and EDG, the compilers that {ref}`the front page <baseline-compilers>` lists. Tidal runs them as programs and records what they do. |
| **The Tidal front ends** | The programs in the directory `frontend/` of the Tidal repository. Each one reads a C++ program and decides what the Tidal specification says about it. `frontend/next/` holds the newer of the two. |

The Tidal specification describes the same language as the ISO
standard, so that a program means the same under both. The ISO
standard remains the normative definition of C++. Tidal does not
replace it.

## The roles of the agents

People and AI agents both work on Tidal. Each agent has one of three
roles, set by what the agent has read.

| Role | What the agent may read | What the agent may write |
|---|---|---|
| **Source reader** | The ISO standard, WG21 papers, cppreference, compiler sources under restrictive licenses, and posts that quote any of these | Nothing in the repository. A source reader gives only summaries in its own words. |
| **Clean-room writer** | Source readers' summaries after the similarity check, the behavior and diagnostics of the baseline compilers, sanitized compiler sources under permissive licenses, and every file of the repository | The Tidal specification, the tests, the tools, and the documentation |
| **Clean-room reader** | Only Tidal's own files: the Tidal specification, the tests, and their results | Only its own exercise directories, and the Tidal front ends |

An agent that reads protected material becomes a source reader for the
rest of its session. It cannot work in the clean room again in that
session.

A source reader's summary describes language behavior, gaps, and
doubts. It contains no code, no quoted or paraphrased text, no proposed
wording, and no section numbers from a source. Before a clean-room
writer reads a summary, the tool `tools/similarity.py` compares the
summary with a reference corpus. A summary with a flagged passage goes
back to the source reader to be reworded.

## The workflow

```{raw} html
<figure class="tidal-diagram">
<svg font-family="sans-serif" viewBox="0 0 840 480" role="img" aria-labelledby="wf-title wf-desc">
  <title id="wf-title">The Tidal workflow</title>
  <desc id="wf-desc">Protected sources reach only source readers. Their summaries pass the similarity check before clean-room writers read them. Writers also use the behavior of the baseline compilers, and write the Tidal specification and tests. Clean-room readers, including the Tidal front ends, read only Tidal's files and send findings back to the writers. The tests run on the compilers, and the results become each page's Evidence.</desc>
  <defs>
    <marker id="wf-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
      <path d="M0,0 L10,5 L0,10 z" class="wf-head" fill="currentColor"/>
    </marker>
  </defs>
  <!-- the clean-room boundary -->
  <line x1="218" y1="16" x2="218" y2="464" class="wf-boundary" stroke="currentColor" stroke-dasharray="6 5"/>
  <text x="34" y="460" class="wf-note" fill="currentColor" font-size="11" font-style="italic">outside the clean room</text>
  <text x="232" y="460" class="wf-note" fill="currentColor" font-size="11" font-style="italic">inside the clean room</text>

  <!-- boxes -->
  <rect x="20" y="40" width="180" height="76" rx="6" class="wf-box wf-protected" fill="none" stroke="currentColor" stroke-width="1.2" stroke-dasharray="5 4"/>
  <text x="110" y="66" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">The ISO standard,</text>
  <text x="110" y="84" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">WG21 papers, other</text>
  <text x="110" y="102" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">protected sources</text>

  <rect x="20" y="190" width="180" height="60" rx="6" class="wf-box" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="110" y="225" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Source readers</text>

  <rect x="240" y="190" width="130" height="60" rx="6" class="wf-box" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="305" y="215" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Similarity check</text>
  <text x="305" y="234" class="wf-label wf-mono" fill="currentColor" font-size="13" text-anchor="middle">tools/similarity.py</text>

  <rect x="400" y="30" width="200" height="60" rx="6" class="wf-box" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="500" y="56" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Baseline compilers</text>
  <text x="500" y="75" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">Clang, GCC, EDG</text>

  <rect x="400" y="190" width="200" height="60" rx="6" class="wf-box wf-accent" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="500" y="225" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Clean-room writers</text>

  <rect x="400" y="340" width="200" height="70" rx="6" class="wf-box wf-accent" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="500" y="368" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">The Tidal specification</text>
  <text x="500" y="388" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">and its tests</text>

  <rect x="640" y="340" width="170" height="70" rx="6" class="wf-box" fill="none" stroke="currentColor" stroke-width="1.2"/>
  <text x="725" y="364" class="wf-label wf-strong" fill="currentColor" font-size="13" text-anchor="middle">Clean-room readers</text>
  <text x="725" y="383" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">exercises and</text>
  <text x="725" y="400" class="wf-label" fill="currentColor" font-size="13" text-anchor="middle">the Tidal front ends</text>

  <!-- arrows -->
  <line x1="110" y1="116" x2="110" y2="188" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <text x="118" y="157" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">read</text>

  <line x1="200" y1="220" x2="238" y2="220" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <text x="219" y="182" class="wf-edge" fill="currentColor" font-size="12" text-anchor="middle">summaries</text>

  <line x1="370" y1="220" x2="398" y2="220" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <text x="385" y="270" class="wf-edge" fill="currentColor" font-size="12" text-anchor="middle">passed</text>

  <line x1="500" y1="90" x2="500" y2="188" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <text x="508" y="135" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">behavior and</text>
  <text x="508" y="151" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">diagnostics</text>

  <line x1="500" y1="250" x2="500" y2="338" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <text x="508" y="290" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">rules, tests,</text>
  <text x="508" y="306" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">questions</text>

  <line x1="600" y1="375" x2="638" y2="375" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>

  <path d="M725,340 L725,220 L602,220" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <text x="733" y="270" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">findings: gaps,</text>
  <text x="733" y="286" class="wf-edge" fill="currentColor" font-size="12" text-anchor="start">doubts, failures</text>

  <path d="M500,410 L500,440 L826,440 L826,60 L602,60" class="wf-arrow" fill="none" stroke="currentColor" stroke-width="1.4" marker-end="url(#wf-arrow)"/>
  <text x="663" y="432" class="wf-edge" fill="currentColor" font-size="12" text-anchor="middle">tests run on the compilers;</text>
  <text x="663" y="454" class="wf-edge" fill="currentColor" font-size="12" text-anchor="middle">results go to each page's Evidence</text>
</svg>
<figcaption>Protected sources reach only source readers. Everything
inside the clean room comes from summaries that passed the similarity
check, from the baseline compilers, and from Tidal's own files.</figcaption>
</figure>
```

A page of the Tidal specification goes through these steps
(`documentation/pipeline.md` in the repository gives each one in
full):

1. **Topic.** A clean-room writer picks the language feature that the
   page will cover.
2. **Probe.** The writer runs small programs on the baseline compilers
   and records what each compiler accepts, rejects, and prints.
3. **Read sources.** The writer reads the sanitized, permissively
   licensed compiler sources where they help, and the source readers'
   summaries that passed the similarity check.
4. **Classify.** The writer sorts the behavior into kinds: required,
   rejected, undefined, unspecified, implementation-defined.
5. **Draft.** The writer writes the page's principles and rules. Each
   rule has an ID, a kind, and test programs that show it.
6. **Questions.** Where the compilers disagree, the writer adopts a
   provisional answer, records it as a question, and goes on.
7. **Similarity check.** `tools/similarity.py` compares the page with
   the reference corpus, and the writer rewrites each flagged passage.
8. **Evidence.** Every test runs on every baseline compiler, and the
   results go into the Evidence segment of the page.

The repository's history records each step. Each step is one commit,
pushed when it is made, and the history is never rewritten.

## How the Tidal specification improves

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
broad. The report passes the similarity check, and a clean-room writer
then probes the compilers and writes the rule independently.

**Closeness to the ISO standard is measured.** Tidal is also an
experiment in how close independent text ends up to the wording of the
ISO standard. The similarity checks that guard each step also measure
that closeness.
