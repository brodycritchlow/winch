## Table of contents 📔

* [`Background`](#background)
* [`Installation`](#installation)
* [`Hysp usages`](#usages)
* [`Hosting custom repo`](#repo)
* [`Packages`](#pkgs)
* [`Support`](#support)
* [`License`](#license)

<a name="bacground"></a>
## Background

<a name="installation"></a>
 ## Installation 📩
    
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
  git clone --depth=1 https://github.com/pwnwriter/hysp --branch=main
  cd hysp
  cargo build --release 
  ```
  Then go to `release` dir and `./hysp` or move the `binary` to your any `$PATH` for instant access from anywhere.
</details>

<a name="usages"></a>
 ## Winch usages
<details> <summary><code>Help menu</code></summary>
  &nbsp;
  
  
  ```bash
  hysp |install|uninstall|search| -h # check for help menu
  ```
<!---![screenshot_2023-11-25_22-37-02](https://github.com/pwnwriter/hysp/assets/90331517/48e6d5be-3174-4aef-8d5e-a9c02c58aaf4)-->

</details>

<details> <summary><code>Installing a pkg </code></summary>
&nbsp;
  
  ```bash
  hysp install <pkg> # use --force to overwrite already installed binary 
  ```
  <!---![screenshot_2023-11-25_22-37-02](https://github.com/pwnwriter/hysp/assets/90331517/48e6d5be-3174-4aef-8d5e-a9c02c58aaf4)-->

</details>


<details> <summary><code>Removing a pkg </code></summary>
&nbsp;
  
  ```bash
  hysp remove <pkg> 
  ```

<!---![screenshot_2023-11-25_22-37-02](https://github.com/pwnwriter/hysp/assets/90331517/48e6d5be-3174-4aef-8d5e-a9c02c58aaf4)-->

</details>

<details> <summary><code>Install remote repository </code></summary>
&nbsp;

  This requires a `config.winch`, you can find more [here](#usages)
  
  ```bash
  hysp install -rp <link>
  ```

<!---![screenshot_2023-11-25_22-37-02](https://github.com/pwnwriter/hysp/assets/90331517/48e6d5be-3174-4aef-8d5e-a9c02c58aaf4)-->

</details>

<a name="repo"></a>
 ## Hosting custom repo

- Winch provies the following configuration, which can be overwritten by defining a `config.winch` file in your repository.
  Here is an example of a config:

  ```
    package
        remote = ""
        aarch = "Architecture"

    local
        home="/home/user/.local/winch/hysp"
        bin="/home/user/.local/winch/hysp/bin/" 
        data="/home/user/.local/share/winch/data/Architecture/" 
    
    install
        "cargo build"
  ```
- Explanation 

| NAME        | DESCRIPTION                                  | DEFAULT                                      |
|-------------|----------------------------------------------|----------------------------------------------|
| remote      | Remote repository URL for the project        | `""`                      |
| aarch       | Target architecture for the project          | `Architecture`                              |
| home        | Where winch builds the project source.                   | `/home/user/.local/winch/hysp`              |
| install     | Installation commands for the project        | `cargo build`                               |

<details> <summary><code>🎄 Tree view of the repo </code></summary>
&nbsp;

  ```bash
.
│
├── src
│   └── x86_64 # Your cpu Architecture 
│       └── config.winch # The config file for your chosen architecture
|   └── universal
│       └── config.winch # The config file for your chosen architecture
```

</details>