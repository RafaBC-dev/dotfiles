#!/bin/bash
HYPRGAMEMODE=$(hyprctl getoption decorations:blur:enabled | awk 'NR==1{print $2}')

if [ "$HYPRGAMEMODE" = 1 ] ; then
    # ENTRAR EN MODO JUEGO
    hyprctl --batch "\
        keyword animations:enabled 0;\
        keyword decoration:drop_shadow 0;\
        keyword decoration:blur:enabled 0;\
        keyword general:gaps_in 0;\
        keyword general:gaps_out 0;\
        keyword general:border_size 1;\
        keyword decoration:rounding 0"
    powerprofilesctl set performance
    notify-send "🚀 Modo Juego: ACTIVADO" "Rendimiento máximo y latencia mínima."
    exit
fi

# SALIR DEL MODO JUEGO (VOLVER A LA NORMALIDAD)
hyprctl reload
powerprofilesctl set balanced
notify-send "🍃 Modo Juego: DESACTIVADO" "Configuración visual restaurada."
