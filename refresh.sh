#!/usr/bin/env bash
set -euo pipefail

ACTIVE_WINDOW=$(xdotool getactivewindow)

xdotool search --classname Navigator search --name "React app" windowactivate --sync key F5

xdotool windowactivate --sync ${ACTIVE_WINDOW}
