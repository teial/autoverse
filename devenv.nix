{ pkgs, lib, config, inputs, ... }:
{
    scripts.install.exec = ''
        cargo install cargo-shuttle
        cargo install cargo-watch
        cargo install cargo-nextest
        cargo install dioxus-cli
    '';

    languages.rust = {
        enable = true;
        channel = "stable";
        targets = [ "wasm32-unknown-unknown" "wasm32-wasi" ];
    };

    packages = lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk; [
        frameworks.CoreServices
        frameworks.SystemConfiguration
        frameworks.Security
        frameworks.Cocoa
    ]) ++ [
        pkgs.libiconv
        pkgs.just
    ];
}
