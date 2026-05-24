# Zed version-lsp

A [version-lsp](https://github.com/skanehira/version-lsp) extension for [Zed](https://zed.dev).

Provides version checking diagnostics for package dependency files:

| File | Registry |
| --- | --- |
| `package.json` | npm |
| `pnpm-workspace.yaml` | npm (pnpm catalogs) |
| `Cargo.toml` | crates.io |
| `go.mod` | Go Proxy |
| `pyproject.toml` | PyPI |
| `.github/workflows/*.yaml` / `.github/actions/*/*.yaml` | GitHub Releases |
| `deno.json` / `deno.jsonc` | JSR |
| `compose.yaml` / `docker-compose.yaml` | Docker Hub / ghcr.io |

## Installation

Install from the Zed extensions registry, or install as a dev extension by cloning this repository and selecting the directory via `zed: install dev extension`.

## Configuration

The language server binary is automatically downloaded from [GitHub Releases](https://github.com/skanehira/version-lsp/releases). You can also specify a custom binary path in your Zed settings:

```json
{
  "lsp": {
    "version-lsp": {
      "binary": {
        "path": "/path/to/version-lsp"
      }
    }
  }
}
```

### Settings

```json
{
  "lsp": {
    "version-lsp": {
      "settings": {
        "cache": {
          "refreshInterval": 86400000
        },
        "registries": {
          "npm": { "enabled": true },
          "crates": { "enabled": true },
          "goProxy": { "enabled": true },
          "pypi": { "enabled": true },
          "github": { "enabled": true },
          "pnpmCatalog": { "enabled": true },
          "jsr": { "enabled": true },
          "docker": { "enabled": true }
        },
        "ignorePrerelease": true
      }
    }
  }
}
```

| Option | Type | Default | Description |
| --- | --- | --- | --- |
| `cache.refreshInterval` | number | `86400000` | Cache refresh interval in milliseconds |
| `registries.npm.enabled` | boolean | `true` | Enable npm registry checks |
| `registries.crates.enabled` | boolean | `true` | Enable crates.io registry checks |
| `registries.goProxy.enabled` | boolean | `true` | Enable Go Proxy registry checks |
| `registries.pypi.enabled` | boolean | `true` | Enable PyPI registry checks |
| `registries.github.enabled` | boolean | `true` | Enable GitHub Releases checks |
| `registries.pnpmCatalog.enabled` | boolean | `true` | Enable pnpm catalog checks |
| `registries.jsr.enabled` | boolean | `true` | Enable JSR registry checks |
| `registries.docker.enabled` | boolean | `true` | Enable Docker Hub / ghcr.io checks |
| `ignorePrerelease` | boolean | `true` | Ignore prerelease versions |

## Development

See the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.

```sh
# Check compilation
cargo check --target wasm32-unknown-unknown
```

## License

MIT