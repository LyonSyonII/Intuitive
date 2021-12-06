## Literals
| Type            | Syntax    | Example   |
|----             |----       |----       |
| Integers        | `NUM`     | `10`      |
| Floats/Decimals | `NUM,NUM` |`10,5`     |
| Strings/Text    | `"TEXT"`  | `"Hello"` |

Integers are treated as Floats for simplicity.

## Sentences
Intuitive provides a set of basic instructions called sentences.  
All sentences end with a `.`, like a normal one would.  
e.g. `Print "Hello World!".`  
A [list of all available sentences](#list-of-sentences) is available below. 

## Synonims
They are what makes Intuitive special.  
Almost all instructions can be written in many ways, exactly the same as a real language.

Each instruction has concise and verbose forms.  
The most idiomatic ones are listed below, but you can mix the core statements with any Linker, allowing for rich expressions.

## Linkers
Linkers are connectors that you can mix in between of phrases to increase readibility or make them more intuitive for newcomers.

Linkers are effectively ignored by the compiler, so they can be used anywhere.

## Variables
All variables must start with an UPPERCASE letter. 
This is to avoid having too much keywords, as every synonim would be a reserved keyword.  
e.g. `Age` (instead of `age`).  
Variables should follow CamelCase formatting.

# List of Sentences
## **Assignation**
Assigns a value to a variable.
 
| <div style="width:200px">Verbose</div> |    <div style="width:200px"> </div>     |
| -------------------------| ---------             |
 Syntax                    | Example               
 `Name is equal to VALUE.` | `Age is equal to 16.` 
 `Name equals VALUE.`      | `Age equals 16`     

| <div style="width:200px">Concise</div> |   <div style="width:200px"> </div>      |
| -------------------------| ---------             |
   Syntax                  | Example               
 `Name = VALUE.`           | `Age = 16.`         

### **Declaration/Definition**
Defines a variable.  
Equivalent to assignation in most contexts, except inside If statements, where it will create a new variable only available there (for examples see If).

| <div style="width:200px">Verbose</div> |    <div style="width:200px"> </div>     |
|---                        |---                    |
| Syntax                    | Example               |
| `Declare Name as VALUE`   | `Declare Age as 16.`  |
| `Define Name as VALUE.`   | `Define Age as 16.`   |

| <div style="width:200px">Concise</div> |   <div style="width:200px"> </div>      |
|---                        |-----                    
| Syntax                    | Example               |
| `Def Name as VALUE.`      | `Def Age as 16.`      |

