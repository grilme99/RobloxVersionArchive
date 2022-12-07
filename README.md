<!-- Allow this file to not have a first line heading -->
<!-- markdownlint-disable-file MD041 no-emphasis-as-heading -->

<!-- inline html -->
<!-- markdownlint-disable-file MD033 -->

<div align="center">

# `🗄️ Roblox Version Archive`

**A simple (and safe!) to consume history of Client and Studio deployment versions.**

</div>

## About

Parsing Roblox's DeployHistory format is troublesome. It can change, information can be added or removed, and multiple downloads of (seemingly) the same Studio or Client version can exist. This repository is structured to be easily consumable by applications in an easy-to-parse JSON format (see blow).

### Safe?

The biggest problem with Roblox's DeployHistory is that it isn't safe to consume. It provides no way to distinguish between Global and China versions, which poses a significant risk to user accounts. If a global Roblox user uses the China version of Roblox, all sorts of weird affects can happen to their account (including deletion).

This archive does not use DeployHistory. Instead, it frequently polls Roblox's `client-version` API for new versions of each deployment option. This is the same API Roblox's official boostrappers use to get the latest version of their applications.

## Format

The current deployment record format looks like this:
```rust
struct DeploymentRecord {
    clientVersion: String,
    bootstrapperVersion: String,

    majorRev: u32,
    version: u32,
    patch: u32,
    changeList: u64,
}
```

Records are stored in a `DeployHistory.json` file as a Map with a string key (corresponding to the `changeList`) and `DeploymentRecord` value.

Like this: `Map<String, DeploymentRecord>`

You can also access the latest deployment info in the `LatestDeploy.json` file.

### Accessing Deployments

This repository is structured to allow straightforward access to whatever deployment you are looking for. Use the following URL as a template:

`https://raw.githubusercontent.com/grilme99/RobloxVersionArchive/main/{DEPLOYMENT_SPACE}/{BINARY_TYPE}/{CHANNEL}/{RECORD}`

The current supported values for each are:

`DEPLOYMENT_SPACE`:
- `Global`
- `China`

`BINARY_TYPE`:
- `WindowsPlayer`
- `WindowsStudio`
- `WindowsStudio64`
- `MacPlayer`
- `MacStudio`

`CHANNEL_NAME`:
- `live`
- `zcanary`
- `zintegration`

`RECORD`:
- `DeployHistory.json`
- `LatestDeploy.json`

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
