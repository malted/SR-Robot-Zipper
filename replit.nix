{ pkgs }: {
	deps = [
		pkgs.rustc
        pkgs.rustup
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
        pkgs.rust-analyzer
        pkgs.unzip
	];
}