#!/bin/bash

# 1. Configurar según el tipo
if [ "$1" == "--out" ]; then
    TITULO="󰓃 SALIDAS"
    TIPO="sink"
    ACTUAL_NAME=$(LC_ALL=C pactl get-default-sink)
elif [ "$1" == "--in" ]; then
    TITULO="󰍬 MICRÓFONOS"
    TIPO="source"
    ACTUAL_NAME=$(LC_ALL=C pactl get-default-source)
else
    exit 1
fi

# 2. Obtener nombres y descripciones de forma sincronizada
# Usamos un truco de 'paste' para tener Name y Description en la misma línea
mapfile -t DISPOSITIVOS < <(LC_ALL=C pactl list ${TIPO}s | grep -E "Name:|Description:" | sed 's/^[ \t]*//' | cut -d: -f2- | sed 's/^[ \t]*//' | paste - - | grep -v "Monitor")

# 3. Construir la lista para Rofi y buscar la fila activa
ITEMS=""
ROW=0
CONTADOR=0

for linea in "${DISPOSITIVOS[@]}"; do
    # Separamos el nombre técnico de la descripción amigable
    n_tecnico=$(echo "$linea" | cut -f1)
    n_desc=$(echo "$linea" | cut -f2)
    
    ITEMS+="$n_desc\n"
    
    # Si este nombre coincide con el actual, guardamos la fila
    if [ "$n_tecnico" == "$ACTUAL_NAME" ]; then
        ROW=$CONTADOR
    fi
    CONTADOR=$((CONTADOR + 1))
done

# 4. Tema de Rofi (Minimalista estilo Orbit)
TEMA="
* { background-color: #1a1b26; text-color: #c0caf5; font: \"JetBrainsMono NF 11\"; }
window { width: 400px; location: north east; y-offset: 50px; x-offset: -15px; border-radius: 16px; border: 2px; border-color: #292e42; }
mainbox { padding: 15px; }
inputbar { padding: 0 0 12px 0; children: [prompt]; }
prompt { text-color: #7dcfff; font: \"JetBrainsMono NF Bold 12\"; }
listview { lines: 6; scrollbar: false; spacing: 6px; }
element { padding: 12px 16px; border-radius: 10px; }
element selected { background-color: #292e42; text-color: #7dcfff; }
element-text { background-color: inherit; text-color: inherit; }
"

# 5. Lanzar Rofi con el índice pre-seleccionado
SELECCION=$(echo -e "$ITEMS" | grep -v "^$" | rofi -dmenu -i -p "$TITULO" -selected-row "$ROW" -theme-str "$TEMA")

# 6. Aplicar el cambio si se seleccionó algo nuevo
if [ -n "$SELECCION" ]; then
    # Buscamos el nombre técnico que corresponde a esa descripción
    NUEVO_NOMBRE=$(LC_ALL=C pactl list ${TIPO}s | grep -B 5 "$SELECCION" | grep "Name:" | tail -n 1 | cut -d: -f2- | sed 's/^[ \t]*//')
    
    if [ "$TIPO" == "sink" ]; then
        pactl set-default-sink "$NUEVO_NOMBRE"
        # Mover streams (música, navegador...) a la nueva salida
        for i in $(LC_ALL=C pactl list short sink-inputs | awk '{print $1}'); do
            pactl move-sink-input "$i" "$NUEVO_NOMBRE" 2>/dev/null
        done
    else
        pactl set-default-source "$NUEVO_NOMBRE"
    fi
    notify-send -h string:x-canonical-private-synchronous:audio "$TITULO" "$SELECCION"
fi
