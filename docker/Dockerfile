FROM ubuntu:24.04

LABEL org.opencontainers.image.source=https://github.com/x-software-com/mxl-player-components
LABEL org.opencontainers.image.description="Build image for MXL crates"
LABEL org.opencontainers.image.licenses="MIT OR APACHE-2.0"

# Base image requirements
RUN apt update \
  && apt install -y --no-install-recommends \
  ca-certificates lsb-release build-essential clang git tar curl wget zip unzip

# Requirements for mxl-investigator, mxl-player-components and mxl-relm4-components
RUN apt update \
  && apt install -y --no-install-recommends \
  linux-tools-common linux-tools-generic xvfb \
  libgtk-4-dev libadwaita-1-dev

# Requirements for mxl-player-components
RUN apt update \
  && apt install -y --no-install-recommends \
  libgstreamer-plugins-base1.0-dev libgstreamer-plugins-good1.0-dev libgstreamer-plugins-bad1.0-dev \
  gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad \
  libgstreamer1.0-dev
