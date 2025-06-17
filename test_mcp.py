#!/usr/bin/env python3
"""
Simple test script for the MCP ImageMagick server
"""
import json
import subprocess
import sys
import os

def send_request(proc, request):
    """Send a JSON-RPC request and get response"""
    request_str = json.dumps(request) + '\n'
    proc.stdin.write(request_str.encode())
    proc.stdin.flush()
    
    response = proc.stdout.readline()
    return json.loads(response)

def main():
    # Start the MCP server
    server_path = "./target/release/mcp-imagemagick"
    if not os.path.exists(server_path):
        print(f"Server not found at {server_path}")
        return 1
    
    proc = subprocess.Popen(
        [server_path],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=False
    )
    
    try:
        # Initialize
        print("Initializing...")
        response = send_request(proc, {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {}
        })
        print("Initialize response:", json.dumps(response, indent=2))
        
        # List tools
        print("\nListing tools...")
        response = send_request(proc, {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list",
            "params": {}
        })
        print("Tools list response:", json.dumps(response, indent=2))
        
        # Check converters
        print("\nChecking converters...")
        response = send_request(proc, {
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {
                "name": "check_converters",
                "arguments": {}
            }
        })
        print("Check converters response:", json.dumps(response, indent=2))
        
        # Test conversion
        print("\nTesting conversion...")
        test_input = "/opt/exp/exp-0614/receipts/IMG_4320.DNG"
        test_output = "/tmp/test_output.webp"
        
        response = send_request(proc, {
            "jsonrpc": "2.0",
            "id": 4,
            "method": "tools/call",
            "params": {
                "name": "convert_dng_to_webp",
                "arguments": {
                    "input_path": test_input,
                    "output_path": test_output,
                    "converter": "darktable"
                }
            }
        })
        print("Conversion response:", json.dumps(response, indent=2))
        
        # Check if output file exists
        if os.path.exists(test_output):
            size = os.path.getsize(test_output)
            print(f"\nSuccess! Output file created: {test_output} ({size} bytes)")
            os.remove(test_output)
        else:
            print("\nError: Output file not created")
        
    finally:
        proc.terminate()
        proc.wait()

if __name__ == "__main__":
    main()