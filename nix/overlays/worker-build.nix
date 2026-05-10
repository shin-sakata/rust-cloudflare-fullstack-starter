# nixpkgs (unstable) の worker-build は 0.7.4 で worker 0.8.x の Cargo.toml を弾くため、
# 0.8.3 を独自 derivation として提供する overlay。
# nixpkgs (pkgs/by-name/wo/worker-build/package.nix) が 0.8 系に追従したら本ファイルと
# flake.nix 側の overlays 指定を削除し、`pkgs.worker-build` をそのまま使う形に戻すこと。
final: prev: {
  worker-build = final.rustPlatform.buildRustPackage (finalAttrs: {
    pname = "worker-build";
    version = "0.8.3";

    src = final.fetchFromGitHub {
      owner = "cloudflare";
      repo = "workers-rs";
      tag = "v${finalAttrs.version}";
      hash = "sha256-sRKQALNYUmzxaqYJCWR8b3yvqg8e4EHe1Cm7vqRx8hU=";
      fetchSubmodules = true;
    };

    cargoHash = "sha256-enePrsTLpiTDxqnFFD38N4amOKY5oHHctPl9RFj2eRo=";

    buildAndTestSubdir = "worker-build";

    meta = {
      description = "Tool to be used as a custom build command for a Cloudflare Workers `workers-rs` project";
      mainProgram = "worker-build";
      homepage = "https://github.com/cloudflare/workers-rs";
      license = with final.lib.licenses; [
        asl20
        mit
      ];
    };
  });
}
