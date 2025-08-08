# Turing Machine Implementation

## Formal Definition

Following
[Cornell University](https://www.cs.cornell.edu/courses/cs4820/2018sp/handouts/turingm.pdf)
notes on Turing machines, we can define what a Turing machine is:

A Turing machine is specified by a finite alphabet $\Sigma$, a finite set of states
$K$ with a special element $s$ (the starting state), and a transition function

$$\delta : K \times \Sigma \rightarrow (K \cup \{halt, yes, no\}) \times \Sigma \times \{\leftarrow, \rightarrow, −\}$$.

It is assumed that $\Sigma$, $K$, $\{halt,yes,no\}$, and $\{\leftarrow, \rightarrow, −\}$ are disjoint sets,
and that $\Sigma$ contains two special elements $\sqcup, \vartriangleright$
representing the start and end of the tape, respectively. We require that for every
$q \in K$, if $\delta(q, \vartriangleright) = (p, \sigma, d)$ then $\sigma = \vartriangleright$
and $d \neq \leftarrow$. In other words, the machine never tries to overwrite the
leftmost symbol on its tape nor to move to the left of it.

And how it operates:

The set $\Sigma \ast$ is the set of all finite sequences of elements of $\Sigma$. When
an element of $\Sigma \ast$ is denoted by a letter such as $x$, then the elements of the
sequence $x$ are denoted by $x_0, x_1, x_2, . . . , x_{n−1}$, where $n$ is the length of $x$.
The length of $x$ is denoted by $|x|$. A configuration of a Turing machine is an ordered
triple $(x, q, k) \in \Sigma \ast \times K \times N$, where $x$ denotes the string on the
tape, $q$ denotes the machine’s current state, and $k$ denotes the position of the
machine on the tape. The string $x$ is required to begin with $\vartriangleright$ and end with
$\sqcup$. The position $k$ is required to satisfy $0 ≤ k < |x|$.
If $M$ is a Turing machine and $(x, q, k)$ is its configuration at any point in time, then
its configuration $(x′, q′, k′)$ at the following point in time is determined as follows. Let
$(p, \sigma, d) = \delta(q, x, k)$. The string $x′$ is obtained from $x$ by changing $x_k$
to $\sigma$, and also appending $\sqcup$ to the end of $x$, if $k = |x| − 1$. The new
state $q′$ is equal to $p$, and the new position $k′$ is equal to $k − 1$, $k + 1$, or $k$
according to whether $d$ is $\leftarrow$, $\rightarrow$, or $−$, respectively. We express
this relation between $(x, q, k)$ and $(x′, q′, k′)$ by writing

$$(x, q, k) \xrightarrow{M} (x′, q′, k′)$$.

A computation of a Turing machine is a sequence of configurations $(x_i, q_i, k_i)$,
where $i$ runs from $0$ to $T$ (allowing for the case $T = \infty$) that satisfies:

- The machine starts in a valid starting configuration, meaning that $q_0 = s$ and $k_0 = 0$.
- Each pair of consecutive configurations represents a valid transition, i.e. for $0 ≤ i < T$,
  it is the case that

  $$(x, q, k) \xrightarrow{M} (x′, q′, k′)$$.

- If $T = \infty$, we say that the computation does not halt.

## Pseudo-code Implementation

WIP
