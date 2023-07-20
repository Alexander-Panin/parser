Parser
======

### Run

`cargo run`

It will try to parse the files in the root/js folder and print one line of output for each file.

### Status

Work in progress - sorry no docs, no tests, some commented println! in the source code.

### Deps

Currently one dependency: crate rayon - just for parsing files in parallel

### Goal

To be honest, I have no idea :) The end goal may change later, for now I guess it's about counting compound tokens for different "js" frameworks.

Hypothesis: we can't count lines of code (because it's not meaningful), but we can count tokens. Let's check it out :)

### Design

The idea was borrowed from double-entry bookkeeping: token should appear once in a sort of true table and secondly in a sort of table of possible tokens. True table in this sense is a vector of tokens after parsing the "js" file, the second table is a vector that grows dynamically according to the last seen true token.

In other words (just from an implementation point of view), it's a tree that extends itself based on the last seen compound token. (e.g. if the matching compound token is "Statement", the tree will replace the matching leaf with a new tree containing the tokens "While", "ForLoop", "If" and so on). In the end we either get an empty tree (which is good) or not :)

### File Structure

The idea is just to make the logic very simple. It's just one possible implementation of parsing logic, so why not make it simple :)

```
atoms
..macros.rs -- just macros for making trees
..grammar.rs -- grammar for parsing js files
..utils.rs -- simple helpers (how without it? :)
..tokens/engine -- just simple tokenizer
audit
..double_entry.rs -- logic for comparing two tables of tokens: real and possible
registry.rs -- sort of a hashtable, but much easier to debug
main.rs -- entry point
```
