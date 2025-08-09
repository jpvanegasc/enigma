# Turing Machine Implementation

## Formal Definition

Following
[Cornell University](https://www.cs.cornell.edu/courses/cs4820/2018sp/handouts/turingm.pdf)
notes on Turing machines, we can define what a Turing machine is:

A Turing machine is specified by a finite alphabet $\Sigma$, a finite set of states
$K$ with a special element $s$ (the starting state), and a transition function

```math
\delta : K \times \Sigma \rightarrow (K \cup \{halt, yes, no\}) \times \Sigma \times \{\leftarrow, \rightarrow, −\} \quad .
```

It is assumed that $\Sigma$, $K$, $`\{halt,yes,no\}`$, and $`\{\leftarrow, \rightarrow, −\}`$ are disjoint sets,
and that $\Sigma$ contains two special elements $\vartriangleright$, $\sqcup$
representing the start and end of the tape, respectively. We require that for every
$k \in K$, if $\delta(k, \vartriangleright) = (p, \sigma, d)$ then $\sigma = \vartriangleright$
and $d \neq \leftarrow$. In other words, the machine never tries to overwrite the
leftmost symbol on its tape nor to move to the left of it.

And how it operates:

The set $\Sigma^\ast$ is the set of all finite sequences of elements of $\Sigma$. When
an element of $\Sigma^\ast$ is denoted by a letter such as $x$, then the elements of the
sequence $x$ are denoted by $x_0, x_1, x_2, . . . , x_{n−1}$, where $n$ is the length of $x$.
The length of $x$ is denoted by $|x|$. A configuration of a Turing machine is an ordered
triple $(x, k, i) \in \Sigma \ast \times K \times N$, where $x$ denotes the string on the
tape, $k$ denotes the machine’s current state, and $i$ denotes the position of the
machine on the tape. The string $x$ is required to begin with $\vartriangleright$ and end with
$\sqcup$. The position $i$ is required to satisfy $0 ≤ i < |x|$.
If $M$ is a Turing machine and $(x, k, i)$ is its configuration at any point in time, then
its configuration $(x′, k′, i′)$ at the following point in time is determined as follows. Let
$(p, \sigma, d) = \delta(k, x_i)$. The string $x′$ is obtained from $x$ by changing $x_i$
to $\sigma$, and also appending $\sqcup$ to the end of $x$, if $i = |x| − 1$. The new
state $k′$ is equal to $p$, and the new position $i′$ is equal to $i − 1$, $i + 1$, or $i$
according to whether $d$ is $\leftarrow$, $\rightarrow$, or $−$, respectively. We express
this relation between $(x, k, i)$ and $(x′, k′, i′)$ by writing

```math
(x, k, i) \xrightarrow{M} (x′, k′, i′) \quad .
```

A computation of a Turing machine is a sequence of configurations $(x_t, k_t, i_t)$,
where $t$ runs from $0$ to $T$ (allowing for the case $T = \infty$) that satisfies:

- The machine starts in a valid starting configuration, meaning that $k_0 = s$ and $i_0 = 0$.
- Each pair of consecutive configurations represents a valid transition, i.e. for $0 ≤ t < T$,
  it is the case that

```math
(x, k, i) \xrightarrow{M} (x′, k′, i′) \quad .
```

- If $T = \infty$, we say that the computation does not halt.

Taking this definition, let's define exactly what is each of the components of the Turing
machine:

- The alphabet $\Sigma$ is a set of characters which the machine accepts. E.g., `{'a', 'b', '1'}`.
- The states $K$ are **names**. In the definition that I'm contemplating, a state doesn't
  encode any information besides an identifier. E.g., `{'state_1', 'MyIncredibleState', '2'}`.
- The transition function $\delta$ encodes the actual ruleset of the Turing machine. It
  specifies how the machine behaves when a particular symbol $\sigma \in \Sigma$ is read
  when the machine is in a specific state $k \in K$. Such a mapping could be

```math
\delta(k=state_1, \sigma=a) = (k=MyIncredibleState, \sigma=b, d=\rightarrow) \\
\delta(k=MyIncredibleState, \sigma=b) = (k=2, \sigma=1, d=-) \\
\delta(k=2, \sigma=1) = (k=halt, \sigma=1, d=\leftarrow) \quad .
```

## Pseudocode Implementation

We can implement a Turing machine $M$ as being an entity that holds the tape $x$, and the
transition function $\delta$, while also keeping track of the current position in the tape
$i:0<=i<=n$ and the state it's currently in $k \in K$:

```
M:
  x
  delta
  i
  k
```

Where `delta` takes a state $k \in K$ and a symbol $\sigma \in \Sigma$ and returns a state
$`k \in (K \cup \{halt, yes, no \} )`$, a direction $`d \in \{ \rightarrow, \leftarrow, -\ \}`$,
and a symbol $\sigma \in \Sigma$:

```
delta(k, s) -> k, d, s
```

Note the implementation for `delta` should include the constraint for not going over the
left boundary of $x$.

The pseudo-code implementation of a Turing machine computation, including defining the
transition function, would be:

```
# Define transition function
delta("k0", '>') := ("k1", '>', +1)
delta("k1", '0') := ("k1", '0', +1)
delta("k1", '1') := (halt, '0', 0)

# Initialize machine
M.x = ['>', '0', '1', 'c']
M.delta = delta
M.i = 0
M.k = "k0"

# Run computation
while M.k != halt:
  result := delta(M.k, M.x[M.i])
  M.k := result[0]
  M.x[M.i] := result[1]
  M.i += result[2]
```

This particular example results in `M.x = ['>', '0', '0', 'c']` when `M.k == halt`.
