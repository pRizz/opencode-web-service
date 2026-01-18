# opencode-web-service
A docker container configuration and scripts for running OpenCode and its web UI in the cloud,
using https://github.com/anomalyco/opencode as the main service in the container.

The goal is to expose the OpenCode web UI publicly so it can be accessed safely from anywhere,
while keeping sessions consistent and persisted on a cloud host (e.g., an EC2 instance). The
service should be hardened against restarts and networking issues where possible.

Future direction: package an installable service (cargo or npm/npx) with a basic setup
walkthrough that installs OpenCode as a system service, and supports easy installation and
uninstallation.
