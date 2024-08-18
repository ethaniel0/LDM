# LDM

## About

LDM, short for "Language Doesn't Matter", is a scripting language that lets you create scripts in whatever syntax you like that compile into whatever language you want, all defined using rules in config files. In other words, when used correctly, it's a universal transpiler - from whatever made-up language you want to whatever real language you want.

## Transpiling

LDM is still a work in progress, so it doesn't fully support transpilation yet.

## Config Files

Config files are written using commonly-written units. Each unit looks like this:
```
<class name> <specifiers (name, etc)> (
    %command_name_1 <command_inner> %
    %command_name_1 <command_inner> %
    ...
)
```

Different class names represent different objects used for compilation. For instance, the `keyword` class name defines a keyword that will be used in compilation, and the `operator` class name defines an operator.

Here's some example config code from the current standard library:

```
keyword for (
    %structure 
        for {typename typename is int} {name var} in {typed int start} %nospace% .. %nospace% {typed int end} %optional %nospace% ..{typed int step}% {block block}
    %
)

operator + (
    %precedence 8 %
    %structure {expression left} + {expression right} %
    %returns left_type %
)

make_variable (
    %structure
        {typename typename} {name varname} %optional = {expression expr}%
    %
)
```

In the first unit in the above code, the `for` keyword is defined. Keywords are defined by their structure. Curly braces within a command (such as structure, in this case) are instructions for what text should be writtin in the source code. This defines that after the for keyword, a type must be supplied that is or can be read as an int, then a variable name. `%nospace%` specifies that the range for a for loop must look like `0..10..1`, and not `0 .. 10 .. 1`. In total, the for keyword structure looks like this:

```
for int i in 0..10 {
    ...more code here
}
```
or with a step,
```
for int i in 0..10..2 {
    ...more code here
}
```

The operator unit defines three "commands". while precedence, structure, and returns are all commands, they are treated like keys in a dictionary, with the items inside being their value. In this example, the + operator is defined with a precedence of 8, binary structure with left and right expressions being added together, and a return type of whatever the left value is.

Lastly, any make_variable unit must define two parameters and one optional (or not optional) parameter: a type called typename, a variable called name, and an optional evaluation expression expr. For this unit, a variable definition might look as such: 
```
int x
int y = 5 + 8
```
If you wanted a more TypeScript-style instantiation, then the unit could look more like this:
```
make_variable (
    %structure
        {name object}: {typename typename} %optional = {expression expr}%
    %
)
```


# Translating Files

Translation files are written much in the same way as config files. Here's an example that would transpile the example `for`, `+`, and variable creation into Python:

```
%target python %
%variable INDENT 4 %

keyword for (
%structure
    for {=var} in range({=start}, {=end} %if(step) , {=step}% ):
        %indent({%INDENT%})% {block}
%
)

operator + (
%structure
    {=left} + {=right}
%
)

make_object standard(
%structure
    {=varname} = %if(expr) {=expr}% %if(not expr) None%
%
)

```
Notice the names for each structure - var, start, end, and step in for; left and right in the + operator; and object and expr in make_object - are used with the `{=<name>} `brace expression.
The `%indent(<#>)%` expression is used to add indents to all lines within a code block, with `{%name%}` being used to retrieve a variable defined at the start of the file. 

