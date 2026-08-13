"""CORTEX security - Cryptographic operations boundary.

All hashing and integrity operations MUST reside within this package.
No other package SHALL perform cryptographic operations directly.

Security Boundary Rules:
- BLAKE3 is used exclusively for integrity hashing
- BLAKE3 is NOT used for encryption, key derivation, or password hashing
- No AES-256 or symmetric encryption in the architecture
"""
