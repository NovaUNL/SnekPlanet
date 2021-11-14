## What is this?

A client library for the [Supernova](https://gitlab.com/claudiop/Supernova/) REST API.

## Installation

- Install maturin
- Run `maturin build` in the repo root.
- Install wheel with `pip install wheel`

# Usage

```python
from supernova import Supernova

client = Supernova()
# VVV Only needed for privileged data VVV
client.set_token("token")
# Or
client.login("username", "password")
departments = client.departments
some_department = department[0]
print(department.name) # Something like "Física" gets printed
department.classes
...
# (Look at the source, should be quite self-explanatory for now)
```

## Important notes

- Very explodey. Trivial to get exceptions out of nowhere. Could get more polish :)
- To be written, but probably the same notes as the Rust client as this is a derivative work.
