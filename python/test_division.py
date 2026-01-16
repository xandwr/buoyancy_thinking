#!/usr/bin/env python3
"""Test the enhanced division physics with Pauli Exclusion, bubble repulsion, and breathing waves."""

import requests
import time
import json

BASE_URL = "http://localhost:3000"

def start_experiment(dividend: float, divisor: float, salinity: float = 2.0):
    """Start a division experiment."""
    resp = requests.post(f"{BASE_URL}/divide", json={
        "dividend": dividend,
        "divisor": divisor,
        "salinity": salinity
    }, timeout=5)
    return resp.json() if resp.ok else None

def wait_for_settlement(max_wait: float = 8.0):
    """Wait for the experiment to settle."""
    start = time.time()
    while time.time() - start < max_wait:
        resp = requests.get(f"{BASE_URL}/divide/status", timeout=5)
        status = resp.json()
        if not status.get("active", True):
            return True
        time.sleep(0.2)
    return False

def get_latest_result():
    """Get the most recent experiment result."""
    resp = requests.get(f"{BASE_URL}/divide/results", timeout=5)
    results = resp.json()
    return results[-1] if results else None

def run_test(dividend: float, divisor: float, expected_remainder: float):
    """Run a single division test."""
    print(f"\n{'='*60}")
    print(f"TEST: {dividend} ÷ {divisor} = {dividend//divisor} remainder {expected_remainder}")
    print('='*60)

    # Start experiment
    start_resp = start_experiment(dividend, divisor)
    if not start_resp:
        print("  ERROR: Failed to start experiment")
        return None

    print(f"  Started: {start_resp.get('message', 'OK')}")

    # Wait for settlement
    if not wait_for_settlement():
        print("  WARNING: Experiment did not settle in time")

    # Get result
    result = get_latest_result()
    if not result:
        print("  ERROR: No result returned")
        return None

    print(f"  Result:")
    print(f"    Quotient:         {result['quotient']}")
    print(f"    Remainder:        {result['remainder']}")
    print(f"    Is Divisible:     {result['is_divisible']}")
    print(f"    Peak Jitter:      {result['peak_jitter']:.4f}")
    print(f"    Velocity Sigma:   {result['velocity_sigma']:.4f}")
    print(f"    Turbulence:       {result['turbulence_energy']:.2f}")
    print(f"    Ticks to Settle:  {result['ticks_to_settle']}")
    print(f"    Node Occupancy:   {result['node_occupancy']}")

    return result

def main():
    print("Testing Enhanced Division Physics")
    print("(Pauli Exclusion + Bubble Repulsion + Breathing Wave)")

    # Test cases
    tests = [
        # (dividend, divisor, expected_remainder)
        (6, 3, 0),   # Clean division: 6÷3=2 r 0
        (7, 3, 1),   # Remainder 1: 7÷3=2 r 1
        (8, 3, 2),   # Remainder 2: 8÷3=2 r 2
        (9, 3, 0),   # Clean division: 9÷3=3 r 0
        (10, 4, 2),  # Remainder 2: 10÷4=2 r 2
    ]

    results = []
    for dividend, divisor, expected_rem in tests:
        result = run_test(dividend, divisor, expected_rem)
        if result:
            results.append({
                "dividend": dividend,
                "divisor": divisor,
                "expected_remainder": expected_rem,
                "actual_remainder": result["remainder"],
                "peak_jitter": result["peak_jitter"],
                "is_divisible": result["is_divisible"],
            })
        time.sleep(0.5)  # Brief pause between tests

    # Summary
    print("\n" + "="*60)
    print("SUMMARY: Peak Jitter by Remainder")
    print("="*60)

    divisible_jitter = [r["peak_jitter"] for r in results if r["expected_remainder"] == 0]
    remainder_jitter = [r["peak_jitter"] for r in results if r["expected_remainder"] > 0]

    if divisible_jitter:
        print(f"  Divisible cases (r=0):   avg jitter = {sum(divisible_jitter)/len(divisible_jitter):.4f}")
    if remainder_jitter:
        print(f"  Remainder cases (r>0):   avg jitter = {sum(remainder_jitter)/len(remainder_jitter):.4f}")

    if divisible_jitter and remainder_jitter:
        ratio = (sum(remainder_jitter)/len(remainder_jitter)) / (sum(divisible_jitter)/len(divisible_jitter) + 0.0001)
        print(f"\n  Jitter Ratio (remainder/divisible): {ratio:.2f}x")
        if ratio > 1.5:
            print("  ✓ SUCCESS: Remainder cases show significantly higher jitter!")
        else:
            print("  ✗ NEEDS TUNING: Jitter difference not significant enough")

if __name__ == "__main__":
    main()
