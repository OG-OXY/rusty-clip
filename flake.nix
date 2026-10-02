{
  description = "Lightning-fast persistent Wayland clipboard daemon & CLI";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    let
      # Defined reusable module that configures the systemd service automatically
      sharedModule = { pkgs, ... }: {
        # Automatically install the package when the module is enabled
        environment.systemPackages = [ self.packages.${pkgs.system}.default ];

        # Automatically manage the background daemon via systemd user services
        systemd.user.services.rusty-clip = {
          description = "Lightning-fast persistent Wayland clipboard daemon";
          wantedBy = [ "graphical-session.target" ];
          partOf = [ "graphical-session.target" ];
          serviceConfig = {
            ExecStart = "${self.packages.${pkgs.system}.default}/bin/rusty-clip daemon";
            Restart = "always";
            RestartSec = "1";
          };
        };
      };
    in
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "rusty-clip";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          # Replace all programs/scripts calling wl-copy to call rusty-clip
          postInstall = ''
            ln -s $out/bin/rusty-clip $out/bin/wl-copy
          '';
        };
      }) // {
        # Exporting the module globally across systems
        nixosModules.default = sharedModule;
        homeManagerModules.default = sharedModule;
      };
}
