# Resources
## Primary resources
The game has 6 main resource types that can be extracted from planets by
the colonies
- Light metals
- Heavy metals
- Precious metals
- Water
- Crude oil
- Silicon


## Secondary resources
Secondary resources are manufactured by buildings in colonies, not mined, 
unlike primary resources
```mermaid
flowchart TB
    pm[Precious metals] & si[Silicon] --> el[Electronics]
    co[Crude oil] --> ke[Kerosene]
    hm[Heavy metals] & pm --> hra[Heat-resistant alloys]
    pm --> sc[Superconductors]
    co --> pl[Plastic]
    pl & lm[Light metals] --> cs[Composites]
    hm & pm --> rp[Radioactive pellets]
```

## Components
Components are manufactured by colonies and are used alongside raw 
secondary resources to build ship modules
```mermaid
flowchart TB
    hra[Heat-resistant alloys] --> en[Engine nozzles]
    sc[Superconductors] & el[Electronics] --> mp[Microprocessors]
    pm[Precious metals] & el --> sn[Sensors]
    lm[Light metals] & rp[Radioactive pellets] --> fl[Fuel rods]
```