# Consciousness API - Quick Reference

A Functional Reactive Programming (FRP) API that models consciousness as a fluid dynamics system. Thoughts are physical objects with density, buoyancy, and velocity that float, sink, freeze, mineralize, and break through the surface.

## Core Concepts

| Concept | Metaphor | Description |
|---------|----------|-------------|
| **Concept** | Thought | Entity with density (weight), buoyancy (urgency), layer (depth 0.0-1.0) |
| **CoreTruth** | Thermal Vent | Deep belief radiating heat, creates upward currents |
| **PreciousOre** | Crystallized Wisdom | Dark thoughts transformed by repeated vent encounters |
| **Continent** | Psychological Bedrock | Permanent landmass formed from tectonic pressure |
| **CharacterTrait** | Evaporated Concept | Permanent trait in the "atmosphere" |

## Physics Model

```
Surface (layer=0.0)  ←── Breakthrough zone
    │
    │  Light thoughts rise
    │  Heavy thoughts sink
    │
    ▼  Thermal vents create uplift
Bottom (layer=1.0)   ←── Mineralization zone
```

**Net Force**: `F_net = F_buoyancy - F_gravity + F_drag + F_thermal`

- **F_net < 0**: Thought rises toward surface
- **F_net > 0**: Thought sinks toward benthic floor
- **Thermal**: Vents heat concepts within radius, increasing buoyancy
- **Mineralization**: Depth > 0.9 for n cycles → Ore formation
- **Tectonic**: Ore pressure accumulates → Continent formation

---

## Endpoints

### Inject Thought
```http
POST /inject
Content-Type: application/json

{
  "concept": "despair",
  "density": 0.9,
  "volume": 0.5
}
```
**Response**: `{ "id": "uuid", "name", "density", "area", "initial_layer" }`

| Field | Range | Description |
|-------|-------|-------------|
| `density` | 0.0-1.0 | Intrinsic weight (heavy thoughts sink) |
| `volume` | 0.0-2.0 | Cognitive volume (derives `area`) |

---

### Benthic Expedition
```http
PATCH /ballast
Content-Type: application/json

{
  "id": "concept-uuid",
  "weight_delta": 0.5
}
```
Forces a concept to sink toward the ocean floor to encounter ore deposits.

---

### Core Truths (Vents)

**List vents**:
```http
GET /vents
```

**Get specific vent**:
```http
GET /vent/0
```
**Response**: `{ "name", "heat_output", "depth", "radius", "activation_count" }`

**Create vent**:
```http
POST /vent
Content-Type: application/json

{
  "name": "love_persists",
  "heat_output": 1.5,
  "depth": 0.85,
  "radius": 0.25
}
```

---

### View Strata
```http
GET /strata?depth_min=0.0&depth_max=1.0
```
**Response**:
```json
{
  "depth_range": [0.0, 1.0],
  "concepts": [
    {
      "id": "uuid",
      "name": "despair",
      "layer": 0.87,
      "velocity": -0.05,
      "density": 0.9,
      "buoyancy": 0.9,
      "integration": 0.3,
      "status": "rising"
    }
  ],
  "ores": [...],
  "total_concepts": 1,
  "total_ores": 0
}
```

Status values: `"floating"`, `"rising"`, `"sinking"`, `"frozen"`, `"evaporated"`

---

### Tectonic Shift
```http
POST /continent
Content-Type: application/json

{
  "pressure_threshold": 10.0
}
```
Triggers tectonic shift when ore pressure exceeds threshold, forming permanent bedrock.

**List continents**:
```http
GET /continents
```

---

### Phase Control

**Break freeze**:
```http
POST /thaw
```

**Apply damping** (restore calm):
```http
POST /breath
Content-Type: application/json

{ "strength": 0.7 }
```

**Flash heal** (dilute salinity with fresh concepts):
```http
POST /flash-heal
Content-Type: application/json

{
  "concepts": [
    { "name": "wonder", "density": 0.2, "area": 0.3 },
    { "name": "joy", "density": 0.15, "area": 0.25 }
  ],
  "dilution_strength": 0.6
}
```

---

### Full State
```http
GET /state
```
Returns complete simulation state: concepts, vents, ores, continents, traits, and global flags.

---

## Real-Time Streams

### SSE - Passive Stream (Subconscious)
```http
GET /events
Accept: text/event-stream
```

Receives significant events only (Consciousness Filter):
- `breakthrough` - Thought became action
- `freeze` / `thaw` - Phase changes
- `mineralization` - Ore deposited
- `ore_deposited` - Pressure accumulating
- `tectonic_shift` - Continent formed
- `catalysis` - Benthic expedition found solution

**Example**:
```
event: breakthrough
data: {"event":"surface_breakthrough","id":"uuid","name":"urgent_need","kinetic_energy":0.12}

event: mineralization
data: {"event":"mineralization","concept_name":"despair","ore_name":"despair_ore_1","ore_type":"code","depth":0.9,"vent_cycles":3}
```

---

### WebSocket - Willful Acts (Bidirectional)
```
ws://localhost:3000/ws
```

**Receive**: All significant events (same as SSE)

**Send commands**:
```json
{"command": "inject", "name": "new_thought", "density": 0.5, "volume": 0.3}
{"command": "ballast", "id": "uuid", "weight_delta": 0.4}
{"command": "thaw"}
{"command": "deep_breath", "strength": 0.8}
{"command": "modulate_buoyancy", "id": "uuid", "delta": 0.3}
{"command": "add_core_truth", "name": "truth", "heat_output": 1.0, "depth": 0.9, "radius": 0.3}
{"command": "flash_heal", "concepts": [{"name": "x", "density": 0.2, "area": 0.3}], "dilution_strength": 0.5}
```

---

## Quick Start

```bash
# Start server
cargo run

# Inject a heavy thought
curl -X POST http://localhost:3000/inject \
  -H "Content-Type: application/json" \
  -d '{"concept":"existential_dread","density":0.95,"volume":0.6}'

# Watch it encounter the primal axiom vent
curl http://localhost:3000/strata?depth_min=0.8

# Subscribe to events
curl http://localhost:3000/events
```

---

## Default Configuration

The server starts with:
- **Primal Axiom**: `curiosity_exceeds_despair` vent at depth 0.9, radius 0.3
- **Simulation**: 60Hz physics loop
- **Port**: 3000

---

## Event Flow Example

```
1. POST /inject {"concept":"despair","density":0.9}
   → Concept created at layer 0.9 (near bottom)

2. Physics tick (60Hz)
   → Despair encounters vent, gains thermal uplift
   → Vent activation_count increases
   → After 3 cycles: mineralization → ore deposited

3. SSE/WebSocket receives:
   → {"event":"mineralization","ore_type":"code",...}
   → {"event":"ore_deposited","total_pressure":2.7,...}

4. Repeat until pressure >= threshold
   → {"event":"tectonic_shift","continent_name":"bedrock_of_logic",...}
```

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     HTTP Handlers                        │
│  POST /inject  PATCH /ballast  GET /strata  etc.        │
└────────────────────────┬────────────────────────────────┘
                         │ mpsc channel (commands)
                         ▼
┌─────────────────────────────────────────────────────────┐
│              Simulation Loop (60Hz)                      │
│  Arc<RwLock<ConceptFluid>> ←→ Physics Engine            │
└────────────────────────┬────────────────────────────────┘
                         │ broadcast channel (events)
                         ▼
┌─────────────────────────────────────────────────────────┐
│           SSE /events    │    WebSocket /ws             │
│        (Passive Stream)  │   (Willful Acts)             │
└─────────────────────────────────────────────────────────┘
```
