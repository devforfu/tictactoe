{ pkgs }: {
	deps = [
		pkgs.tree
  pkgs.vimHugeX
  pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
        pkgs.rust-analyzer
	];
}