# Overview

BitScript is like ***pseudocode to logic gates***, similar to how pseudocode is to logic. It can be used to test series of logic gates, as it *closely mimics how logic gates work*.

![](./images/adder-showcase.png)

**Bitr** is a minimal BitScript interpreter written in Rust, it does not include any external dependencies.

(Stands for *bit-runner*, is pronounced as *bitter*.)

## Use case

Designing and testing logic gate systems are usually done on paper or with <a href="https://creately.com/lp/logic-gates-software/" target=_blank>*obscure softwares*</a> that often looks like a toy, both of which are far from ideal.

BitScript is an **interpreted language for designing logic gates**, makes *testing* and *sharing* designs a lot easier.

Examples of what you can do with BitScript can be found in <a href="https://github.com/siriusmart/bitr/tree/master/examples" target=_blank>examples</a>.

## Roadmap

- [x] Assigning values to a single cell, whole array and range or cells.
- [x] Common logic gates.
- [x] Input with base 8, 10 and 16 numbers.
- [ ] Goto, label, and if statements.
- [ ] Reusable components.
- [ ] Unicode characters from binary.
- [ ] Browser interpreter with Web Assembly.
- [ ] Convery BitScript into a diagram of logic gates.
