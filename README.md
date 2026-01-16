# Buoyancy Thinking

A fluid dynamics simulation that models consciousness as a physical medium. Thoughts are buoyant objects that float, sink, freeze, mineralize, and break through the surface based on their intrinsic properties.

## Overview

This project implements a Functional Reactive Programming (FRP) API where:

- **Concepts** (thoughts) have density, buoyancy, and velocity
- **Core Truths** (deep beliefs) act as thermal vents radiating heat
- **Precious Ores** form when dark thoughts cycle through vents
- **Continents** (psychological bedrock) emerge from accumulated ore pressure
- **Character Traits** crystallize when concepts evaporate at the surface

The simulation runs at 60Hz, with real-time event streaming via SSE and WebSocket.

## Quick Start

```bash
# Build and run
cargo run

# Inject a thought
curl -X POST http://localhost:3000/inject \
  -H "Content-Type: application/json" \
  -d '{"concept": "curiosity", "density": 0.3, "volume": 0.5}'

# Watch events
curl http://localhost:3000/events
```

## Division Experiments (Analog Computing)

The fluid can perform arithmetic using acoustic physics:

```bash
# Compute 7 / 3 using standing waves
curl -X POST http://localhost:3000/divide \
  -H "Content-Type: application/json" \
  -d '{"dividend": 7, "divisor": 3, "salinity": 2.0}'

# Wait for bubbles to settle
sleep 6

# Get result
curl http://localhost:3000/divide/results | jq '.[-1]'
```

**How it works**:
1. The **divisor** creates standing wave nodes at acoustic intervals
2. **Bubbles** (dividend) are injected and seek stable nodes
3. **Pauli Exclusion** limits each node to `quotient` bubbles
4. **Lennard-Jones Repulsion** prevents bubble stacking
5. **Remainder** bubbles can't find homes, creating measurable **jitter**

The `peak_jitter` metric distinguishes divisible from remainder cases:
- Clean division (r=0): Low jitter (~0.5)
- Remainder (r>0): High jitter (~8-12)

## Physics Model

```
Surface (layer=0.0)  <-- Breakthrough zone, evaporation
    |
    |  Light thoughts rise (low density)
    |  Heavy thoughts sink (high density)
    |
    v  Thermal vents create uplift
Bottom (layer=1.0)   <-- Mineralization zone
```

**Forces**:
- Buoyancy: `F = (target_layer - current_layer) * density`
- Drag: `F = -0.5 * viscosity * v^2 * Cd * area`
- Thermal: Vents push nearby concepts upward
- Wave: Standing waves attract bubbles to acoustic nodes
- Lennard-Jones: `F = 4e[(s/r)^12]` repulsion between bubbles

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/inject` | POST | Add a new thought |
| `/ballast` | PATCH | Force benthic expedition |
| `/strata` | GET | View concepts at depth range |
| `/vents` | GET | List thermal vents |
| `/vent` | POST | Create new core truth |
| `/continents` | GET | List formed continents |
| `/continent` | POST | Trigger tectonic shift |
| `/thaw` | POST | Break freeze state |
| `/breath` | POST | Apply calming damping |
| `/flash-heal` | POST | Dilute salinity |
| `/state` | GET | Full simulation state |
| `/events` | GET | SSE event stream |
| `/ws` | WS | WebSocket bidirectional |
| `/divide` | POST | Start division experiment |
| `/divide/status` | GET | Experiment progress |
| `/divide/results` | GET | Completed results |

See [API.md](API.md) for detailed documentation.

## Key Concepts

### Salinity
Accumulated "knowledge density" that affects how thoughts move. High salinity makes the fluid more viscous, dampening motion.

### Turbulence
When Reynolds number exceeds threshold, the fluid becomes chaotic. Turbulence energy cascades into smaller eddies, eventually dissipating as "integration" (understanding).

### Freeze
A thought stuck at the surface too long causes a system-wide freeze. All other thoughts are blocked until thaw occurs.

### Mineralization
Heavy thoughts cycling through thermal vents transform into precious ores:
- `Art` - High area concepts
- `Code` - Low density concepts  
- `Writing` - High integration concepts
- `Insight` - Many vent cycles

### Tectonic Shift
When ore pressure exceeds threshold, a permanent continent forms from the accumulated wisdom, reshaping the mental landscape.

## Project Structure

```
src/
  api/
    handlers/     # HTTP endpoint handlers
    routes.rs     # Route definitions
  runtime/
    simulation_loop.rs  # 60Hz physics loop
  simulation/
    fluid.rs      # Core ConceptFluid physics
    concept.rs    # Thought entities
    standing_wave.rs  # Division experiment physics
    ...
  state/
    events.rs     # Event types for streaming
    commands.rs   # Command types for control
```

## License

MIT
