The printing module depends on the printing device which it's responsibility to print most of the output the cli will spit out, and to format it correctly, some examples for output it expected to print:  
1. Text, paragraphs.
2. Errors, suggestions.
3. Lines (like in the help output).
4. Extras like: spaces, taps, headers.
5. And much more (if needed).
# Templates
Printing will be divided between multiple templates (structures); the high and middle level ones in addition for the templates them self's, also have a **low costume** function with the responsibility to give it the printing device, and it will handle the rest; but for the low level templates, their functions are **high costume**.
- See [[Low and high costume]] to understand what is high and low costume functions.