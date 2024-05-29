# Exegraph

A simple tiny rust no_std no alloc graph library with a fixed number of inputs and outputs for each node.

Each data sink can have at most one data source connected to it, but every data source can output its data to as many sinks as there are.

The library currently has a propagation delay which might be of use in some situations (such as when making a circuit simulator), but might also be a hinderance, depending on your needs. I might add more versions of a similar struct to the library as I need them. It is primarily designed to be used for audio applications where everything is a signal of the same type, but it can easily be modified to support other funcitonality as it's only 100 lines of code at the moment. 

