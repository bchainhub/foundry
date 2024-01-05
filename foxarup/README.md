# `foxarup`

Update or revert to a specific Foxar branch with ease.

## Installing

```sh
curl -L https://foxar.paradigm.xyz | bash
```

## Usage

To install the **nightly** version:

```sh
foxarup
```

To install a specific **version** (in this case the `nightly` version):

```sh
foxarup --version nightly
```

To install a specific **branch** (in this case the `release/0.1.0` branch's latest commit):

```sh
foxarup --branch release/0.1.0
```

To install a **fork's main branch** (in this case `transmissions11/foxar`'s main branch):

```sh
foxarup --repo transmissions11/foxar
```

To install a **specific branch in a fork** (in this case the `patch-10` branch's latest commit in `transmissions11/foxar`):

```sh
foxarup --repo transmissions11/foxar --branch patch-10
```

To install from a **specific Pull Request**:

```sh
foxarup --pr 1071
```

To install from a **specific commit**:

```sh
foxarup -C 94bfdb2
```

To install a local directory or repository (e.g. one located at `~/git/foxar`, assuming you're in the home directory)

##### Note: --branch, --repo, and --version flags are ignored during local installations.

```sh
foxarup --path ./git/foxar
```

---

**Tip**: All flags have a single character shorthand equivalent! You can use `-v` instead of `--version`, etc.

---
