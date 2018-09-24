# Ticket

### About Ticket

![](https://img.shields.io/badge/version-beta-EB6EA5.svg)

Unique ID Generator

The ID generated from Ticket only occupies 12 bytes.

- 4-byte from unix timestamp,
- 3-byte from machine id(`/sys/class/dmi/id/product_uuid`),
- 2-byte from current process id, and
- 3-byte counter which starting with a random value.

### License
[BSD 3-Clause License](LICENSE) Copyright (c) 2018, Hanaasagi
