## calc-cli

This is a simple calculator that evaluates expressions like the following:

```
1 + 2 - 3
1 + 2 + 3
16 + 2 - 2
14.5 - 2.5
```

The Shunting-yard algorithm is used to convert the expression to Reverse Polish Notation (RPN) and then the RPN is evaluated. In my implementation, I am not taking into account the precedence of operators or parentheses.
