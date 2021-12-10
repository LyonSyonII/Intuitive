Intuitive is a programming language focused on being easy to learn and understand even for people that has never programmed before.  

To achieve this, instructions (called [_Sentences_](#sentences)) are made to be written as close as possible to real languages (like english or spanish).  
For example, C style `int sum = 5 + 6;` can be written as `Sum is equal to 5 plus 6.` 

Also, for the experienced users, almost all Sentences have a Concise form, becoming the example above `Sum = 5 + 6.`

If you want to know how the language is implemented, check the [Implementation Details](#implementation-details) section.

# Installation
## Prerrequisites
The transpiler is made with Rust, so its toolchain is required.

### Windows
The easiest way to install Rust is using Chocolatey, a windows package manager.
To install Chocolatey run this command in an Admin Powershell
```
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
```
Then use Chocolatey to install the Rust toolchain
```
choco install rust
```

Finally, download the last version of Intuitive from the Releases page and extract it wherever you want.
If you want a GUI version, download the Intuitive-GUI package.

### Linux & MacOS
Just run the next command in a terminal window,
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
download the last version of Intuitive from the Releases page and extract it wherever you want.
If you want a GUI version, download the Intuitive-GUI package.

# Usage
## CLI
Open a console and compile the file following the next syntax
```
intuitive INPUT OUTPUT
```
for example, to compile a file named "HelloWorld.iv" I would write
```
intuitive HelloWorld.iv HelloWorld
```
**Important** Do NOT write any extension on the output file, Intuitive automatically adds the correct extensions.


# Documentation
## Literals
Literals are all values known at compile-time. 
Integers are treated as Floats for simplicity. 
**Important, floats must be written with a `,` not a `.`**
| Type            | Syntax    | Example   |
| --------------- | --------- | --------- |
| Integers        | `NUM`     | `10`      |
| Floats/Decimals | `NUM,NUM` | `10,5`    |
| Strings/Text    | `"TEXT"`  | `"Hello"` |
To add variable values to a String just write them separated by spaces:
```
Eggs = 25.
Text = "I have " Eggs " eggs". (Will end up being "I have 25 eggs")
```

## Variables
All variables must start with an UPPERCASE letter.  
This is to avoid having too much keywords, as every synonym would be a reserved keyword.  
E.g. `Age` (instead of `age`).  

Variables should follow CamelCase formatting.

## Types
Some Sentences allow you to specify a type.
Types are a guide to the compiler on how to interpret a value.

For example, the type of `52` is `Natural/Int`.
| Type                        | Example |
| --------------------------- | ------- |
| `Integer/Int`               | 677     |
| `Number/Real/Decimal/Float` | 3,1416  |
| `Text/Word/String`          | "Heya!" |

## Comments
The compiler will ingnore anything in between parethesis `()`.
This means that operations cannot have parenthesis in them, so do your calculations accordingly.
```
Age = 18. (Get age)

(Check age and print message)
If Age is larger or equal to 18 then: Print "You can enter".
If it isn't: Print "You are not old enough". 
```

## Sentences
Intuitive provides a set of basic instructions called sentences.  
All sentences end with a `.`, like a normal one would.  
E.g. `Print "Hello World!".`  
A [list of all available sentences](#list-of-sentences) is available below. 

## Synonyms
They are what makes Intuitive special.  
Almost all instructions can be written in many ways, exactly the same as a real language.

Each instruction has concise and verbose forms.  
The most idiomatic ones are listed below, but you can mix the core statements with any Linker, allowing for rich expressions.

## Linkers
Linkers are connectors that you can mix in between of phrases to increase readability or make them more intuitive for newcomers.

Linkers are effectively ignored by the compiler, so they can be used anywhere.

## **List of Sentences**
When a word is inside parenthesis `()`, it means it can be omitted, as it's a linker.

### **Assignation**
Assigns a value to a variable.  

Declares it automatically in case it does not exist.  

Variable types are inferred automatically.

| Syntax                        | Example                    |
| ----------------------------- | -------------------------- |
| `Name (is) equal (to) VALUE.` | `Age (is) equal (to) 16. ` | 
| `Name equals VALUE.     `     | `Age equals 16       `     |
| `Name = VALUE.          `     | `Age = 16.           `     |

### **Declaration/Definition**
Declares a variable.  

Equivalent to [assignation](#assignation) in most contexts, except inside If statements, where it will create a new variable only available there (for examples see [If](#if)).

| Syntax                  | Example              |
| ----------------------- | -------------------- |
| `Declare Name as VALUE` | `Declare Age as 16.` |
| `Define Name as VALUE.` | `Define Age as 16.`  |
| `Dec Name as VALUE`     | `Dec Age as 16.`     |
| `Def Name as VALUE.`    | `Def Age as 16.`     |

`Declare` can also be used to create an empty variable and assign a value to it when you want.  
If used this way, the assign sentence will not declare the variable again.
```
Declare Age.
...
Age = 16.
```

### **Operators**
#### **Add**
| Syntax             | Example    |
| ------------------ | ---------- |
| `VALUE plus VALUE` | `1 plus 2` | 
| `VALUE + VALUE`    | `1 + 2`    |

#### **Substract**
| Syntax              | Example     |
| ------------------- | ----------- |
| `VALUE minus VALUE` | `1 minus 2` |
| `VALUE - VALUE`     | `1 - 2`     |

#### **Multiply**
| Syntax                        | Example               |
| ----------------------------- | --------------------- |
| `VALUE multiplied (by) VALUE` | `1 multiplied (by) 2` | 
| `VALUE times VALUE`           | `1 times 2`           |
| `VALUE mul VALUE`             | `1 mul 2`             |
| `VALUE * VALUE`               | `1 * 2`               |

#### **Divide**
| Syntax                     | Example            |
| -------------------------- | ------------------ |
| `VALUE divided (by) VALUE` | `1 divided (by) 2` | 
| `VALUE div by VALUE`       | `1 div by 2`       |
| `VALUE * VALUE`            | `1 / 2`            |

### **Assignment Operators**
Assignment operators allow you to abbreviate expressions like 
```
Sum = 0. 
Sum = Sum + 1.
```
into a single line
```
Add 1 to Sum.
```
They are also much clearer than the typical `+=`, which I've found to be really confusing for newcomers.

#### **Add**
| Syntax                   | Example          |
| ------------------------ | ---------------- |
| `Add VALUE to Variable.` | `Add 1 to Eggs.` |

#### **Substract**
| Syntax                         | Example                |
| ------------------------------ | ---------------------- |
| `Substract VALUE to Variable.` | `Substract 1 to Eggs.` |
| `Sub VALUE to Variable.`       | `Sub 1 to Eggs.`       |

#### **Multiply**
| Syntax                        | Example               |
| ----------------------------- | --------------------- |
| `Multiply Variable by VALUE.` | `Multiply Eggs by 3.` |
| `Mul Variable by VALUE.`      | `Mul Eggs by 3.`      |

#### **Divide**
| Syntax                      | Example             |
| --------------------------- | ------------------- |
| `Divide Variable by VALUE.` | `Divide Eggs by 2.` | 
| `Div Variable by VALUE.`    | `Div Eggs by 2.`    |

### **Print**
Prints a Value or Variable to the console.  
It will add a newline automatically at the end of the print.
You can also concatenate Values/Variables for a prettier printing.
To do so, just write them separated by spaces.
| Syntax            | Example            |
| ----------------- | ------------------ |
| `Print VALUE.`    | `Print 56.`        |
| `Print Variable.` | `Print Eggs.`      |
| `Print OPERATION` | `Print Eggs + 56.` |
Example:
```
Eggs = 12.
Boxes = 3.
Print "Number of Eggs: " Eggs.
Print "Eggs per box: " Eggs/Boxes.
```

### **Read**
Reads input from the user and stores it in a Variable.
The variable [type](#types) can be specified with `as TYPE`.
Read optional messages can only be a simple String, use Print if you want to output a more complex message with formatting.
| Syntax                                        | Example                               |
| --------------------------------------------- | ------------------------------------- |
| `Read Variable as TYPE.`                      | `Read Eggs as Num.`                   |
| `Read Variable.`                              | `Read Eggs.`                          |
| `Read "Optional Message" Variable (as TYPE).` | `Read "Enter number of eggs: " Eggs.` |
Example:
```
(Read marks)
Read "Enter your Maths mark: " Maths.
Read "Enter your Literature mark: " Literature.
Read "Enter your History mark: " History.
(Print average)
Print "Your average mark is: " Maths/3 + Literature/3 + History/3.
```

### **Compound Sentences/Control Flow**
Any compound expression (composed of more sentences inside) can be expressed in two ways, with a comma separator `,` or a dash separator `-`.
You can also use `->` as a dash separator.

**Important**, dash-separated sentences must end with a `.` like all sentences.

Comma-separated sentences cannot have other compound expressions inside, while dash-separated ones can.

If-like expressions (`if`, `else` and `else if`) cannot be nested, as it would be pretty difficult to understand and it's not possible with a simple syntax like this.
Instead, using [functions](#functions) is encouraged.

#### **Comparations**
Comparations are used in conditions, and allow to check if a variable or value is equal `=`, smaller `<` or larger `>` than another.

##### Equal
| Syntax                       | Example                                          |
| ---------------------------- |:------------------------------------------------ |
| `A (is) equal (to) B`        | `VerticalDist is equal to HorizontalDist`        |
| `A (is the) same (as the) B` | `VerticalDist is the same as the HorizontalDist` |
| `A = B`                      | `VerticalDist = HorizontalDist`                  |
| `A == B`                     | `VerticalDist == HorizontalDist`                 | 

##### Smaller
| Syntax                    | Example                                   |
| ------------------------- | ----------------------------------------- |
| `A (is) smaller (than) B` | `Chris is smaller than Robert`            |
| `A (is) lower (than) B`   | `Temperature is lower than 0`             |
| `A (has) less (than) B`   | `StudentsList has less than TeachersList` | 

#### **If**
Checks a condition and executes the code inside if it is true.

Both `then` and `:` can be ommitted, but only if the other one is used. 
| Syntax                                                 | Example                                                                                                       |
|:------------------------------------------------------ |:------------------------------------------------------------------------------------------------------------- |
| `If CMP then: FIRST_SENTENCE, SECOND SENTENCE, etc.`   | `If Eggs <= 0 then: Print "You don't have any eggs left", Print "Please, buy more eggs".`                     |
| `If CMP then 1st, 2nd, 3rd, etc.`                      | `If CMP then Print "You don't have any eggs left", Print "Please, buy more eggs".`                            |
| `If CMP: 1st, 2nd, 3rd, etc.                       `   | `If Eggs <= 0: Print "You don't have any eggs left", Print "Please, buy more eggs". `                         |
|                                                        |                                                                                                               |
| <pre>If CMP then:<br>- 1st<br>- 2nd<br>- etc.</pre>    | <pre>If Eggs <= 0 then: <br>- Print "You don't have any eggs left" <br>- Print "Please, buy more eggs".</pre> |
| <pre>If CMP then:<br>- 1st,<br>- 2nd,<br>- etc.</pre>  | <pre>If Eggs <= 0 then: <br>- Print "You don't have any eggs left", <br>- Print "Please, buy more eggs".</pre>      |
| <pre>If CMP then:<br>-> 1st<br>-> 2nd <br>-> etc.</pre> | <pre>If Eggs <= 0 then: <br>-> Print "You don't have any eggs left" <br>-> Print "Please, buy more eggs".</pre>     |

#### **Else if**
Executes when the `if` statement right before it fails and a condition is met.
| Syntax                                                                   | Example                                                                                                                                                                                        |
| ------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <pre>If...<br>Else if CMP then:<br>- 1st<br>- 2nd<br>- etc.</pre>        | <pre>If Eggs > 0 then: Print "You have eggs".<br>Else if Eggs <= 0 then: <br>- Print "You don't have any eggs left" <br>- Print "Please, buy more eggs".</pre>                                 |
| <pre>If...<br>Else but CMP:<br>- 1st<br>- 2nd<br>- etc.</pre>            | <pre>If Eggs <= 0 then: <br>- Print "You don't have any eggs left" <br>- Print "Please, buy more eggs".</pre>                                                                                  | 
| <pre>If...<br>If not but CMP:<br>- 1st<br>- 2nd<br>- etc.</pre>          | <pre>If Eggs > 0: Print "You have eggs".<br>If not but Eggs <= 0: <br>- Print "You don't have any eggs left" <br>- Print "Please, buy more eggs".</pre>                                        |
| <pre>If...<br>If it isn't and/but CMP:<br>- 1st<br>- 2nd<br>- etc.</pre> | <pre>If Eggs is larger than 0: Print "You have eggs".<br>If it isn't and Eggs is smaller or equal than 0: <br>- Print "You don't have any eggs left" <br>- Print "Please, buy more eggs".</pre> |

#### **Else**
Same as `else if`, but without a condition.
|                                                                                                                                      Syntax | Example                                                                                                                                                     |
| -------------------------------------------------------------------------------------------------------------------------------------------| ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
|                                                                                  <pre>If...<br>Else then:<br>- 1st<br>- 2nd<br>- etc.</pre> | <pre>If Eggs > 0 then: Print "You have eggs".<br>Else then: <br>- Print "You don't have any eggs left" <br>- Print "Please, buy more eggs".</pre>           |
|                                                                                     <pre>If...<br>If not:<br>- 1st<br>- 2nd<br>- etc.</pre> | <pre>If Eggs > 0: Print "You have eggs".<br>If not: <br>- Print "You don't have any eggs left" <br>- Print "Please, buy more eggs".</pre>                   |
|                                                                                <pre>If...<br>If it isn't:<br>- 1st<br>- 2nd<br>- etc.</pre> | <pre>If Eggs is larger than 0: Print "You have eggs".<br>If it isn't: <br>- Print "You don't have any eggs left" <br>- Print "Please, buy more eggs".</pre> |
| <pre>If...<br>Else if...<br>If none:<br>- 1st<br>- 2nd<br>- etc.</pre> | <pre>If Eggs > 1: Print "You have many eggs".<br>Else if Eggs == 1: Print "You have one egg".<br>If none:<br>- Print "You don't have any eggs left" <br>- Print "Please, buy more eggs".</pre>   |