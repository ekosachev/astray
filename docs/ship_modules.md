# Ship modules
Each ship in the game has different modules. Although, you can outfit them 
in any way you want, some modules are necessary:
- Reactor
- Sublight sustainers
- RCS thrusters
- Fuel tank
- Flight computer

## Creating modules
When creating a new module blueprint, you balance the parameters of the 
module against each other.

For example, when creating a new **Fusion reactor**, you have 3 parameters:
- Internal volume
- Temperature
- Containment force

All of them influence the energy output that the reactor will have. 
Internal volume and temperature are directly proportional to energy output, 
while the containment force is inversely proportional to it.

The parameters influence each other, in this case: 
- Higher internal volume leads to lower temperature, while 
  requiring less containment force 
- Higher internal temperature requires greater containment force
- Higher containment force can allow greater internal volume

Also, the secondary characteristics of the module are influenced by parameters:
- Higher internal volume leads to greater mass and lowers the 
  damage-induced meltdown chance
- Higher temperature makes an overload-induced meltdown more likely
- Higher containment force requires more energy, so the part of the energy 
  produced gets spent on keeping the reactor alive

## List of ship modules
### Reactors
#### Fusion reactor
Generates energy for the ship

Parameters:
- Internal volume
- Temperature
- Containment force

Primary characteristic: Power generation

Secondary characteristics:
- Overload-induced meltdown chance
- Damage-induced meltdown chance
- Support power requirement

### Sublight sustainers
#### Ion drive
Provide thrust for moving inside a solar system

Parameters:
- Potential difference
- Nozzle throughput
- Ionization power

Primary characteristic: Power (the one you measure in Watts)
Secondary characteristics:
- Power consumption
- Efficiency
- Fuel consumption

### RCS thrusters
#### Ion thrusters
Provide force to manoeuvre the ship

See [ion drive](#ion-drive) for parameters, primary and secondary 
characteristics

### Fuel tanks
#### Gas storage
Provides storage for various gases used by Ion drive, Ion thrusters and 
Fusion reactors
Each tank can only hold one type of gas

Parameters:
- Volume

Primary characteristic: Capacity (measured as mass stored)
