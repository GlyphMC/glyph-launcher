{
  description = "Glyph Launcher Tauri Shell";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
	nixgl.url = "github:guibou/nixGL";
  };
  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
	nixgl,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay) nixgl.overlay];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rust-analyzer"];
        };
        mkLibraryPath = packages: pkgs.lib.makeLibraryPath packages;
        runtimeLibs = with pkgs; [
          libGL
          mesa
          vulkan-loader
          alsa-lib
          libpulseaudio
          udev
          xorg.libX11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libXinerama
          xorg.libXfixes
          xorg.libXext
          xorg.libXxf86vm
          glfw3-minecraft
          openal
          stdenv.cc.cc.lib
          flite
        ];
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs;
            [
              openssl
              gobject-introspection
              at-spi2-atk
              atkmm
              cairo
              gdk-pixbuf
              glib
              glib-networking
              gtk3
              harfbuzz
              librsvg
              libsoup_3
              pango
              webkitgtk_4_1
              libayatana-appindicator

              # Java
              zulu8
              zulu17
              zulu

              # Audio libs (additional to runtimeLibs)
              pipewire
              libjack2

              # Controller support (additional to runtimeLibs)
              libusb1
            ]
            ++ runtimeLibs;
          nativeBuildInputs = with pkgs; [
            pkg-config
            rustToolchain
            cargo-tauri
            nodejs
            pnpm
            eza
            fd
            mesa-demos # Provides glxinfo for debugging
            pciutils # Provides lspci for hardware detection
            xorg.xrandr # Required by LWJGL
          ];
          shellHook = ''
                        export GIO_MODULE_DIR="${pkgs.glib-networking}/lib/gio/modules"
                        export RUST_SRC_PATH="${rustToolchain}/lib/rustlib/src/rust/library"
                        export LD_LIBRARY_PATH="${mkLibraryPath runtimeLibs}:$LD_LIBRARY_PATH"
            			alias ls=eza
                        alias find=fd
          '';
        };
      }
    );
}
