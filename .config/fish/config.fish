# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# --- VARIABLES DE ENTORNO ---
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
set -gx EDITOR micro
set -gx VISUAL micro
# Intentamos usar Zen, si no, fallback a Firefox (común en Linux)
if type -q zen-browser
    set -gx BROWSER zen-browser
else
    set -gx BROWSER firefox
end

# --- AÑADIR RUTAS AL PATH ---
# fish_add_path es inteligente: solo añade si el directorio existe y no es duplicado
fish_add_path $HOME/.cargo/bin
fish_add_path $HOME/.local/bin

# --- ALIAS DE CONVENIENCIA ---
alias dotsync="python $HOME/dotfiles/install.py"

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# SET VALUES
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
set fish_greeting
set VIRTUAL_ENV_DISABLE_PROMPT "1"
set -x SHELL /usr/bin/fish

# Use bat for man pages (Solo si bat está instalado)
if type -q bat
    set -xU MANPAGER "sh -c 'col -bx | bat -l man -p'"
    set -xU MANROFFOPT "-c"
end

# Hint to exit PKGBUILD review in Paru
set -x PARU_PAGER "less -P \"Press 'q' to exit the PKGBUILD review.\""

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# ENVIRONMENT SETUP (GUARDADO CONTRA ERRORES)
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# Carga de entornos específicos (Solo si los archivos existen)
# Esto corrige los errores que viste en la VM
if test -f ~/.cargo/env.fish
    source ~/.cargo/env.fish
end

if test -f ~/.local/bin/env.fish
    source ~/.local/bin/env.fish
end

# Apply .profile
if test -f ~/.fish_profile
    source ~/.fish_profile
end

# Add depot_tools to PATH
if test -d ~/Applications/depot_tools
    fish_add_path ~/Applications/depot_tools
end

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# STARSHIP PROMPT (Solo si está instalado)
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
if status --is-interactive
    if type -q starship
        source (starship init fish --print-full-init | psub)
    else
        echo "⚠️ Starship no encontrado. Instálalo para ver el prompt personalizado."
    end
end

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# FUNCTIONS (Bang-bang, history, backup, etc.)
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
function __history_previous_command
    switch (commandline -t)
        case "!"
            commandline -t $history[1]; commandline -f repaint
        case "*"
            commandline -i !
    end
end

function __history_previous_command_arguments
    switch (commandline -t)
        case "!"
            commandline -t ""
            commandline -f history-token-search-backward
        case "*"
            commandline -i '$'
    end
end

if [ "$fish_key_bindings" = fish_vi_key_bindings ];
    bind -Minsert ! __history_previous_command
    bind -Minsert '$' __history_previous_command_arguments
else
    bind ! __history_previous_command
    bind '$' __history_previous_command_arguments
end

function history
    builtin history --show-time='%F %T '
end

function backup --argument filename
    cp $filename $filename.bak
end

function cleanup
    while pacman -Qdtq
        sudo pacman -R (pacman -Qdtq)
        if test "$status" -eq 1
            break
        end
    end
end

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# USEFUL ALIASES
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

# Replace ls con eza (Solo si eza existe)
if type -q eza
    alias ls 'eza -al --color=always --group-directories-first --icons'
    alias lsz 'eza -al --color=always --total-size --group-directories-first --icons'
    alias la 'eza -a --color=always --group-directories-first --icons'
    alias ll 'eza -l --color=always --group-directories-first --icons'
    alias lt 'eza -aT --color=always --group-directories-first --icons'
    alias l. 'eza -ald --color=always --group-directories-first --icons .*'
end

# Replace cat con bat
if type -q bat
    abbr cat 'bat --style header,snip,changes'
end

# Paru/Yay logic
if not test -x /usr/bin/yay; and test -x /usr/bin/paru
    alias yay 'paru'
end

# Navegación y utilidades
alias .. 'cd ..'
alias ... 'cd ../..'
alias .... 'cd ../../..'
alias gitpkg 'pacman -Q | grep -i "\-git" | wc -l'
alias grep 'ugrep --color=auto'
alias grubup 'sudo update-grub'
alias hw 'hwinfo --short'
alias ip 'ip -color'
alias upd '/usr/bin/garuda-update'
alias please 'sudo'

# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
# EJECUCIÓN AL INICIO
# ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
if status --is-interactive && type -q fastfetch
    fastfetch
end
