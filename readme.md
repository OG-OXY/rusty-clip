# rusty-clip 🦀📋

A lightning-fast, persistent Wayland clipboard daemon and CLI written in Rust. Designed as a drop-in replacement for the traditional `wl-copy` ecosystem to eliminate random truncations, dropped text blocks, and weird character corruption.

## Why rusty-clip?

If you spend a lot of time in terminal environments, tiling window managers, and power-user text editors like Neovim, you've likely hit the wall with standard Wayland clipboard utilities:
* **Random Truncations:** Yanking large blocks of text only to find the last 25% silently chopped off.
* **Ghost Characters:** Random artifacts or broken formatting sneaking into your pastes.
* **Dropped Payload Silences:** Selections failing under load or desyncing when passing large strings between apps.

`rusty-clip` bypasses these brittle pipelines with a robust background daemon and direct socket/binary communication that actually keeps up with your workflow.

## Features

* **Drop-In Replacement:** Includes a symlink shim for `wl-copy` so existing scripts and bindings work seamlessly without rewriting your setup.
* **Zero-Drop Reliability:** Handles massive text blocks (even multi-thousand-line buffers) cleanly from top to bottom without cutting out.
* **Native Neovim Integration:** Talks directly to Neovim via `vim.g.clipboard` for lightning-fast responsiveness(can set to use it directly with vim options, but the shim/symlink to wl-copy replaces the need to do this.)
```lua
vim.g.clipboard = {
  name = 'rusty-clip-custom',
  copy = {
    ['+'] = 'rusty-clip',
    ['*'] = 'rusty-clip',
  },
  paste = {
    -- Assuming you have a paste mechanism or use wl-paste (which can also be shimmed)
    ['+'] = 'wl-paste --no-newline',
    ['*'] = 'wl-paste --no-newline',
  },
  cache_enabled = 0,
}
```
* **NixOS & Home Manager Ready:** Ships with native module exports for effortless, zero-configuration deployment and automatic `systemd.user` service management.

## Installation & NixOS Module

Add `rusty-clip` as an input in your system's flake, and import its module globally:

```nix
{
  inputs.rusty-clip.url = "github:OG-OXY/rusty-clip"; # or fork it and add your githubusername/rusty-clip
  
  outputs = { self, nixpkgs, rusty-clip, ... }@inputs: {
    nixosConfigurations.nixos = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ./config.nix
        rusty-clip.nixosModules.default
      ];
    };
  };
}
