{ pkgs, ... }:

{
  packages = with pkgs; [
    awscli
    aws-vault
    terraform
    docker
    colima
  ];
}
