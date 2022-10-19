{ sources ? import ./nix/sources.nix
, pkgs ? import sources.nixpkgs { overlays = [(import sources.rust-overlay)]; }
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    niv
    nodejs
    rust-bin.stable.latest.default
    miniserve
  ];
}

