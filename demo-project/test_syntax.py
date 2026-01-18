#!/usr/bin/env python3
"""
Test Python file for syntax highlighting
"""

import os
import sys
from typing import List, Dict, Optional

class EmbeddedDevice:
    def __init__(self, name: str, address: int = 0x1000):
        self.name = name
        self.base_address = address
        self.registers: Dict[str, int] = {}
        self.enabled = False
    
    def read_register(self, offset: int) -> int:
        """Read a register value"""
        addr = self.base_address + offset
        # Simulate register read
        return self.registers.get(f"reg_{offset:04x}", 0)
    
    def write_register(self, offset: int, value: int) -> None:
        """Write a register value"""
        if not self.enabled:
            raise RuntimeError("Device not enabled")
        
        addr = self.base_address + offset
        self.registers[f"reg_{offset:04x}"] = value & 0xFFFF
        print(f"Write 0x{value:04x} to {self.name}@0x{addr:08x}")

def main():
    devices: List[EmbeddedDevice] = []
    
    # Create some test devices
    for i in range(3):
        device = EmbeddedDevice(f"Device_{i}", 0x1000 + i * 0x100)
        device.enabled = True
        devices.append(device)
    
    # Test register operations
    for device in devices:
        try:
            device.write_register(0x00, 0x1234)
            value = device.read_register(0x00)
            assert value == 0x1234, f"Register mismatch: {value} != 0x1234"
        except Exception as e:
            print(f"Error with {device.name}: {e}")

if __name__ == "__main__":
    main()