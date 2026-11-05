{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
	NIX_ENFORCE_PURITY = 0;
  packages = with pkgs; [
    qemu
    OVMF
		gcc-arm-embedded
		gdb
  ];

  shellHook = ''
    export OVMF_FD=${pkgs.OVMF.fd}/FV/OVMF.fd
    echo "QEMU ready. UEFI firmware at: $OVMF_FD"
		fish
  '';
}
