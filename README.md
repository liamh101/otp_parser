# OTP Parser

A Simple 2FA generator script. Pass a secret with optional additional digits and timesteps.

## Build

The rust compiler is required to build the executable.

```
cargo build -r
```

Executable found in `target/release`

## Example

```
./opt_parser -s JBSWY3DPK5XXE3DE
```

## Help

```
./opt_parser -h
```
