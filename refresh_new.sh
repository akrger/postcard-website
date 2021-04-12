#!/usr/bin/env bash
set -euo pipefail

NAME="$1"

ACTIVE_WINDOW=$(xdotool getactivewindow)

xdotool search "$NAME" windowactivate --sync key F5

xdotool windowactivate --sync ${ACTIVE_WINDOW}
