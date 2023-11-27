## Table of contents 📔

- [Table of contents 📔](#table-of-contents-)
- [Background](#background)
- [Installation](#installation)
- [Winch usages](#winch-usages)
- [Hosting custom repo](#hosting-custom-repo)
- [Progress](#progress)

<a name="bacground"></a>
## Background

<a name="installation"></a>
 ## Installation
    
  <details> <summary><code>Cargo</code></summary>

- Using [crates.io](https://crates.io/crates/winch)
  ```bash
  cargo install winch
  ```
- Using [binstall](https://github.com/cargo-bins/cargo-binstall)
  ```bash
  cargo binstall winch
  ```

  > **Note** ⚠️
  > This requires a working setup of rust/cargo & binstall.
  </details>
  <details> <summary><code>Source</code></summary>
  &nbsp;
 
  ```bash
  git clone --depth=1 https://github.com/brodycritchlow/winch --branch=main
  cd winch
  cargo build --release 
  ```
  Then go to `release` dir and `./winch` or move the `binary` to your any `$PATH` for instant access from anywhere.
</details>

<a name="usages"></a>
 ## Winch usages
<details> <summary><code>Help menu</code></summary>
  &nbsp;
  
  
  ```bash
  winch |install|uninstall| -h # check for help menu
  ```
<!---![screenshot_2023-11-25_22-37-02](https://github.com/pwnwriter/winch/assets/90331517/48e6d5be-3174-4aef-8d5e-a9c02c58aaf4)-->

</details>

<details> <summary><code>Installing a pkg </code></summary>
&nbsp;
  
  ```bash
  winch install <pkg> # use --force to overwrite already installed binary 
  ```
  <!---![screenshot_2023-11-25_22-37-02](https://github.com/pwnwriter/winch/assets/90331517/48e6d5be-3174-4aef-8d5e-a9c02c58aaf4)-->

</details>


<details> <summary><code>Removing a pkg </code></summary>
&nbsp;
  
  ```bash
  winch remove <pkg> 
  ```

<!---![screenshot_2023-11-25_22-37-02](https://github.com/pwnwriter/winch/assets/90331517/48e6d5be-3174-4aef-8d5e-a9c02c58aaf4)-->

</details>

<details> <summary><code>Install remote repository</code></summary>
&nbsp;

  This requires a `config.winch`, you can find more [here](#repo)
  
  ```bash
  winch install -rp <link>
  ```

<!---![screenshot_2023-11-25_22-37-02](https://github.com/pwnwriter/winch/assets/90331517/48e6d5be-3174-4aef-8d5e-a9c02c58aaf4)-->

</details>

<a name="repo"></a>
 ## Hosting custom repo

- Winch provies the following configuration, which can be overwritten by defining a `config.winch` file in your repository.
  Here is an example of a config:

  ```
    package
        remote = ""

    local
        home="/home/user/.local/winch/winch"
    
    install
        "cargo build"
  ```
- Explanation 

| NAME        | DESCRIPTION                                  | DEFAULT                                      |
|-------------|----------------------------------------------|----------------------------------------------|
| remote      | Remote repository URL for the project        | `""`                      |
| home        | Where winch builds the project source.       | `/home/user/.local/winch/winch`              |
| install     | Installation commands for the project        | `cargo build`                               |

<details> <summary><code>🎄 Tree view of the repo </code></summary>
&nbsp;

  ```bash
.
│
├── winch
│   └── config.winch # The config file for your chosen architecture
```

</details>

<a name="progress"></a>
## Progress

#### Installation
- [ ] Winch grabs the source code for the specified package. [#1](https://github.com/brodycritchlow/winch/issues/1)
- [ ] Winch grabs the source code for the remote repository package. [#1](https://github.com/brodycritchlow/winch/issues/1)
- [ ] Winch uses the `./winch/config.winch` to install and add binary to `$PATH`. [#2](https://github.com/brodycritchlow/winch/issues/2)
- [ ] Winch adds the binary to `config.winch['home']` for ease of access. [#2](https://github.com/brodycritchlow/winch/issues/2)
- [ ] Winch uses specified mirrors through `-mirror=` option. [#3](https://github.com/brodycritchlow/winch/issues/3)

#### Removal
- [ ] Winch finds the binary in `config.winch['home']` and removes it.
- [ ] Winch finds any folders that were created by the package and removes it. (`--clean`)

#### Documentation
- [ ] Add a custom help screen for a user-friendly interface. 
- [ ] Add documentation for each command.
- [ ] Add more in-depth documentation about remote repos.

#### Winch Other
- [ ] Add a way to search through packages, that are indexed in winch.
- [ ] Add a way to add package repos, that make it easier to install remote repos.
