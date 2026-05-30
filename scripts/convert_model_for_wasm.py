#!/usr/bin/env python3
"""
Convert trained model weights to the format expected by WASM AI service.
The WASM code expects simple flat arrays of weights, but the trained models
have a complex structure with layer-specific weights.
"""

import json
import sys
from pathlib import Path


def flatten_layer_weights(layer_data):
    """Flatten layer weights into a single array."""
    weights = []

    for key, value in layer_data.items():
        if key.endswith(".weight"):
            if isinstance(value, list):
                for row in value:
                    weights.extend(row)
            else:
                weights.extend(value)
        elif key.endswith(".bias"):
            if isinstance(value, list):
                weights.extend(value)
            else:
                weights.extend([value])

    return weights


def convert_model_format(input_path, output_path):
    """Convert model from complex format to simple flat arrays."""
    print(f"Converting {input_path} to {output_path}")

    with open(input_path, "r") as f:
        model = json.load(f)

    value_weights = []
    policy_weights = []

    if "value_network" in model:
        value_weights = flatten_layer_weights(model["value_network"])
        print(f"Value network weights: {len(value_weights)} values")

    if "policy_network" in model:
        policy_weights = flatten_layer_weights(model["policy_network"])
        print(f"Policy network weights: {len(policy_weights)} values")

    simple_model = {
        "value_network": {"weights": value_weights},
        "policy_network": {"weights": policy_weights},
    }

    with open(output_path, "w") as f:
        json.dump(simple_model, f, indent=2)

    print(f"✅ Converted model saved to {output_path}")
    return len(value_weights), len(policy_weights)


def main():
    if len(sys.argv) != 3:
        print(
            "Usage: python convert_model_for_wasm.py <input_model.json> <output_model.json>"
        )
        sys.exit(1)

    input_path = sys.argv[1]
    output_path = sys.argv[2]

    if not Path(input_path).exists():
        print(f"Error: Input file {input_path} does not exist")
        sys.exit(1)

    try:
        value_count, policy_count = convert_model_format(input_path, output_path)
        print("✅ Conversion complete!")
        print(f"   Value weights: {value_count}")
        print(f"   Policy weights: {policy_count}")
    except Exception as e:
        print(f"❌ Error converting model: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
