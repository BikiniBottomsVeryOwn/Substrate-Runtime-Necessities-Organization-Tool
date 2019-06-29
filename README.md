# Substrate-Runtime-Necessities-Organization-Tool
Taken from: https://www.parity.io/what-is-substrate/ -Jack Fransham

Took the key points that came after "a simple explanation of what you'd need to implement in order to get a full blockchain up and running" and used them to create an architectural/organizational tool for a fundamentally correct runtime. 

Meant for use with Substrate Core. 

//(Shawn Tabrizi)https://www.shawntabrizi.com/substrate-collectables-workshop/#/1/creating-a-module
For each module, we:

    Import the Rust file containing the module
    Implement its Trait
    Include the module into the construct_runtime! macro
    
    


