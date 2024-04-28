# Buildings
Buildings are constructed in colonies and provide the empire with resources 
either mined, or produced from other resources or generate a particular 
resource or ability by the fact of its existence

## Building types
### Mines
Mines generate resources on their own. Each mine generates all [primary 
resources](resources.md#primary-resources), though when building a mine a 
table, dictating its efficiency gets generated.

Such table might look something like this:

| Resource        | Efficiency (units per RT) |
|-----------------|---------------------------|
| Light metals    | 10                        |
| Heavy metals    | 7                         |
| Precious metals | 3                         |
| Water           | 21                        |
| Crude oil       | 16                        |
| Silicon         | 24                        |

**RT** means Resource Tick, which is a "fps" analog for updating the amount 
of a particular resource player has (in other words, resource amounts are 
updated not on every in-game tick, but on every resource tick). By default,
RT happens every 10 in-game ticks.

On the scale of the game, planet's resources are considered inexhaustible.

### Factories
Factories produce [secondary resources](resources.md#secondary-resources)
using primary resources. Every RT factories check, if enough of the primary 
resource is present and if so, convert it into a respective secondary resource

| Primary resources                    | Secondary resources     |
|--------------------------------------|-------------------------|
| 1x Precious metals + 5x Silicon      | 3x Electronics          |
| 10x Crude oil                        | 5x Kerosene             |
| 3x Heavy metals + 2x Precious metals | 4x Heat-resistant alloy |
| 4x Precious metals                   | 1x Superconductors      |
| 7x Crude oil                         | 2x Plastics             |
| 5x Plastics + 5x Light metals        | 8x Composites           |
| 2x Heavy metals + 1x Precious metals | 1x Radioactive pellets  |

### Ship components factories
Ship components factories work like regular factories, but they take primary 
and secondary resources and combine them into components

| Resources                                | Component          |
|------------------------------------------|--------------------|
| 15x Heat-resistant alloys                | 1x Engine nozzle   |
| 1x Superconductors + 4x Electronics      | 2x Microprocessors |
| 2x Precious metals + 3x Electronics      | 1x Sensors         |
| 3x Light metals + 4x Radioactive pellets | 2x Fuel rods       |

### Spaceports
Spaceports provide the ability for your ships to land on the planet and 
take off from it, you only need 1 spaceport per planet, as they have no 
limit on the ships landed there

### Dry docs
Dry docs are used to start building ships, that then take off and the 
construction is finished in outer space.

In the dry dock only the following modules are built:
- Hull
- Sublight sustainers
- Reactor(s)
- Fuel tanks
- RCS thrusters
- Flight computer

Each dry dock can only house one ship at a time
