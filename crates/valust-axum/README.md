# Valust Utils for Axum

<center>

<img alt="Crate Version" src="https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fgithub.com%2FEmbers-of-the-Fire%2Fvalust-rs%2Fraw%2Frefs%2Fheads%2Fmain%2FCargo.toml&query=%24.workspace.package.version&prefix=v%20&style=for-the-badge&label=version">&emsp;
<img alt="GitHub top language" src="https://img.shields.io/github/languages/top/embers-of-the-fire/valust-rs?style=for-the-badge&color=%23FF9B07">&emsp;
<a href="https://crates.io/crates/valust-axum">
    <img alt="Crates.io Downloads (recent)" src="https://img.shields.io/crates/dr/valust-axum?style=for-the-badge">
</a>&emsp;
<a href="https://crates.io/crates/axum">
    <img alt="Axum Dependency Version" src="https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fgithub.com%2FEmbers-of-the-Fire%2Fvalust-rs%2Fraw%2Frefs%2Fheads%2Fmain%2FCargo.toml&query=%24.workspace.dependencies.axum.version&prefix=v%20&style=for-the-badge&label=axum&color=%23c83895
    ">
</a>

</center>

---

This crate offers multiple utilities for the [`axum`](https://crates.io/crates/axum).

## Feature list

- [extractors](#extractors)

## Extractors

Extractors are stored in the `valust_axum::extractor` module.

| Extractor      | Language                   | Backend                    | `Content-Type`                      |
| -------------- | -------------------------- | -------------------------- | ----------------------------------- |
| `ValidJson`    | [JSON][json]               | [`axum/json`][axum/json]   | `application/json`                  |
| `ValidSonic`   | [JSON][json]               | [`sonic-rs`][sonic-rs]     | `application/json`                  |
| `ValidForm`    | `x-www-form`               | [`axum/form`][axum/form]   | `application/x-www-form-urlencoded` |
| `ValidMsgPack` | [`Message Pack`][msg-pack] | [`rmp-serde`][rmp-serde]   | `application/msgpack`               |
| `ValidXml`     | [XML][xml]                 | [`quick-xml`][quick-xml]   | `application/xml`                   |
| `ValidCbor`    | [CBOR][cbor]               | [`ciborium`][ciborium]     | `application/cbor`                  |
| `ValidYaml`    | [YAML][yaml]               | [`serde_yaml`][serde_yaml] | `application/yaml`                  |
| `ValidToml`    | [Toml][toml]               | [`toml`][toml-lib]         | `application/toml`                  |
| `ValidRon`     | Ron                        | [`ron`][ron-lib]           | `application/ron`                   |

[json]: https://www.json.org/json-en.html
[axum/json]: https://docs.rs/axum/latest/axum/struct.Json.html
[axum/form]: https://docs.rs/axum/latest/axum/struct.Form.html
[sonic-rs]: https://crates.io/crates/sonic-rs
[msg-pack]: https://msgpack.org/
[rmp-serde]: https://crates.io/crates/rmp-serde
[xml]: https://developer.mozilla.org/en-US/docs/Web/XML
[quick-xml]: https://crates.io/crates/quick-xml
[yaml]: https://yaml.org/
[serde_yaml]: https://crates.io/crates/serde_yaml
[toml]: https://toml.io/en/
[toml-lib]: https://crates.io/crates/toml
[ron-lib]: https://crates.io/crates/ron
[cbor]: https://cbor.io/
[ciborium]: https://crates.io/crates/ciborium
