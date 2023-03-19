{pkgs, ...}: {
  packages = with pkgs; [
    openssl
    pkg-config
    p7zip
  ];
  languages.rust.enable = true;
}
