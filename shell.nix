{
  pkgs ? (
    import <nixpkgs> {
      config.allowUnfree = true;
    }
  ),
  userShell ? "fish",
}:

let
  pkgConfigPath = "$PKG_CONFIG_PATH:/usr/lib64/pkgconfig:/usr/lib/pkgconfig:/usr/share/pkgconfig";
  pkgConfigWrapper = pkgs.writeShellScriptBin "pkg-config" ''
    PKG_CONFIG_PATH=${pkgConfigPath} ${pkgs.pkg-config}/bin/pkg-config $@
  '';
in
(pkgs.buildFHSEnv {
  name = "mxl-crates";
  targetPkgs =
    pkgs:
    (with pkgs; [
      pkgConfigWrapper
      vscode # Explicitly install vscode to run the non FSH version to inherit all environment variables
      pkgs.${userShell}

      appimage-run
      fish
      htop
      eza
      ripgrep
      bat
      dust
      fd
      ouch
      gitFull
      stdenv
      gcc
      valgrind
      python3Full
      pipx
      just
      meson
      gdb
      ninja
      pkg-config
      gtk4.dev
      vulkan-loader.dev
      libdrm.dev
      glib.dev
      pango.dev
      harfbuzz.dev
      cairo.dev
      gdk-pixbuf.dev
      librsvg.dev
      libadwaita.dev
      gst_all_1.gstreamer
      gst_all_1.gstreamer.dev
      gst_all_1.gst-vaapi
      gst_all_1.gst-vaapi.dev
      gst_all_1.gst-libav
      gst_all_1.gst-libav.dev
      gst_all_1.gst-plugins-base
      gst_all_1.gst-plugins-base.dev
      gst_all_1.gst-plugins-good
      gst_all_1.gst-plugins-good.dev
      gst_all_1.gst-plugins-bad
      gst_all_1.gst-plugins-bad.dev
      gst_all_1.gst-plugins-ugly
      gst_all_1.gst-plugins-ugly.dev
      gst_all_1.gst-plugins-rs
      gst_all_1.gst-plugins-rs.dev
      gst_all_1.gst-editing-services
      gst_all_1.gst-editing-services.dev
      gst_all_1.gst-devtools
      ffmpeg_6-full.dev
      nvidia-vaapi-driver
      libepoxy.dev
      graphene.dev
      wayland.dev
      kdePackages.wayland.dev
      kdePackages.wayland-protocols
      wayland-scanner
      xorg.xorgproto
      xorg.libX11.dev
    ]);

  runScript = pkgs.writeScript "init.sh" ''
    # By default the GDK backend is set to Wayland on NixOS.
    # This fixes an issue with NVIDIA/GTK4/GStreamer (gtk4paintablesink) under Wayland, where the playback is very slow and choppy.
    # Check in the future, if this issue still exists, so we can remove this workaround.
    export GDK_BACKEND=x11

    # Set the Cargo home directory to avoid conflicts with other projects and different compiler and library versions.
    export CARGO_HOME="${builtins.toString ./.}/.cargo"

    export PKG_CONFIG_PATH="${pkgConfigPath}"
    export PKG_CONFIG_EXECUTABLE="$(which pkg-config)"

    export SHELL="/usr/bin/${userShell}"
    exec ${userShell}
  '';
}).env
