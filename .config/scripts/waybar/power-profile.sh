#!/bin/bash

# 1. Detectar perfil actual para el índice
CURRENT=$(powerprofilesctl get | xargs)
case "$CURRENT" in
    "performance") ROW=0 ;;
    "balanced")    ROW=1 ;;
    "power-saver") ROW=2 ;;
    *)             ROW=1 ;;
esac

# 2. Lanzar Rofi (Tus coordenadas: y:20, x:-15)
opcion=$(echo -e "🚀 Rendimiento\n⚖️ Balanceado\n🍃 Ahorro" | rofi -dmenu -i \
    -p "Perfil de Energía" \
    -selected-row "$ROW" \
    -theme "~/.config/rofi/submenus.rasi" \
    -theme-str 'window { location: north east; y-offset: 20px; x-offset: -15px; width: 300px; }')

# 3. Aplicar perfil
case "$opcion" in
    "🚀 Rendimiento")
        powerprofilesctl set performance
        notify-send "Energía" "Modo Rendimiento activado"
        ;;
    "⚖️ Balanceado")
        powerprofilesctl set balanced
        notify-send "Energía" "Modo Balanceado activado"
        ;;
    "🍃 Ahorro")
        powerprofilesctl set power-saver
        notify-send "Energía" "Modo Ahorro activado"
        ;;
esac
