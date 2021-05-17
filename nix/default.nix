{ sources ? import ./sources.nix }:
with
{
  overlay = self: super:
    {
      niv = import sources.niv {};
    };
};
import sources.nixpkgs
  { overlays = [ overlay ]; config = {}; }
