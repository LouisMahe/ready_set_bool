# Ready Set Bool (42 School Project)

This project contains severals functions manipulating logical/boolean formulas. I used tihs project to start learning rust.
Some crates contain quite simple functions that are described into the `lib.rs`.
Some functions will display a logical formula in a tree form thank to the [text_trees](https://docs.rs/text_trees/latest/text_trees/) crate.
Every crate comes with a main.rs to test.

## Notations
Here is the list of the logical operators used in the functions inputs. The formula are writen in Polish notation.
+ '1' : True
+ '0' : False
+ '!' : Negation
+ '&' : And
+ '|' : Or
+ '>' : Imply
+ '=' : Equivalent
+ Any uppercase letter is a boolean variable

## Boolean Evaluator
 This function will receive a logical formula with operators and assigned variables and returns its value. It will build
 a binary tree from the input and evaluate the formula by descending throught the tree.
 
 ![boolean_eval_example](/pict/bool_eval.png)

 ## Truth Table
 This function will take a logical formula containing variables and return its truth table. It make use of the former function.
 
 ![truth_table_example](/pict/truth_table.png)

 ## Negation Normal Form
 This function will rewrite a formula in [negation normal form](https://en.wikipedia.org/wiki/Negation_normal_form).
 To do so, equivalence will be turned into dual implications, then implications will be rewriten. Thirdly De Morgan's laws
 are used to propagate negation towards variable and at last double negations are simplified.
 
 ![nnf_example](/pict/negation_normal_form.png)

 ## Conjunctive Normal Form (cnf)
 This function will turn a formula into one of its [CNF form](https://en.wikipedia.org/wiki/Conjunctive_normal_form).
 To avoid lenghty formulas the [Quine-McCluskey algorithm](https://en.wikipedia.org/wiki/Quine%E2%80%93McCluskey_algorithm) is used.
 
 ![cnf_example](/pict/cnf.png)

 ## Map_u16
 Both these crates contain two functions. The first one is an injection from `[0, 2^16-1]x[0, 2^16-1]` into `[0,1]`. The other function will take a float as input and returns its eventual pair of antecedent if it exists (relative to the first function).
 The first crate use a inelegant but visual friendly mapping. The second is inspired by [Hilbert's curve](https://en.wikipedia.org/wiki/Hilbert_curve)
