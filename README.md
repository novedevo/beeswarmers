# all glory to the higher dimensional bee jiggler

turns a slice of f64 values into a vec of jiggled bees, suitable for zero-overlap plotting in your favourite graph engine. 
doesn't jiggle in the primary dimension, so the first value of each array-point in the returned vec will be identical to the corresponding value of the input slice.
for more about beeswarm plots, see https://www.rhoworld.com/i-swarm-you-swarm-we-all-swarm-for-beeswarm-plots-0/
