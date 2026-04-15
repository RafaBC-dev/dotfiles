import os
import shutil
import subprocess
from pathlib import Path

# Configuración de rutas usando Path (POO)
HOME = Path.home()
REPO_ROOT = HOME / "dotfiles"
CONFIG_REPO = REPO_ROOT / ".config"
CONFIG_SYS = HOME / ".config"

# Otros mapeos específicos que no están en .config
EXTRA_DOTFILES = {
    REPO_ROOT / "assets" / "fonts": HOME / ".local" / "share" / "fonts",
}

def install_packages():
    pkg_file = REPO_ROOT / "packages.txt"
    if pkg_file.exists():
        confirm = input("📦 ¿Deseas instalar los paquetes de packages.txt? (s/n): ").lower()
        if confirm == 's':
            print("🚀 Instalando paquetes con paru...")
            with open(pkg_file, "r") as f:
                pkgs = [l.strip() for l in f if l.strip() and not l.startswith("#")]
            subprocess.run(["paru", "-S", "--needed", "--noconfirm"] + pkgs)
    else:
        print("⚠️ No se encontró packages.txt")

def link_item(src, dest):
    """Crea un enlace simbólico, eliminando el destino si ya existe."""
    dest.parent.mkdir(parents=True, exist_ok=True)
    
    if dest.exists() or dest.is_symlink():
        if dest.is_dir() and not dest.is_symlink():
            shutil.rmtree(dest)
        else:
            dest.unlink()
            
    os.symlink(src, dest)
    print(f"✅ Enlazado: {dest.relative_to(HOME)}")

def make_scripts_executable():
    """Busca archivos en la carpeta scripts y les da permisos +x."""
    scripts_path = CONFIG_SYS / "scripts"
    if scripts_path.exists():
        print("🔓 Asegurando permisos de ejecución en scripts...")
        for script in scripts_path.rglob("*"):
            if script.is_file():
                # Equivale a chmod 755
                script.chmod(0o755)

def create_links():
    print("🔗 Generando enlaces simbólicos...")
    
    # 1. Automatizar todo lo que esté en dotfiles/.config
    if CONFIG_REPO.exists():
        for item in CONFIG_REPO.iterdir():
            link_item(item, CONFIG_SYS / item.name)

    # 2. Enlaces específicos (Fuentes, etc.)
    for src, dest in EXTRA_DOTFILES.items():
        if src.exists():
            link_item(src, dest)

def main():
    print(f"--- Instalador de Dotfiles de {HOME.name} ---")
    
    # 1. Instalación de software
    install_packages()
    
    # 2. Creación de enlaces
    create_links()
    
    # 3. Permisos de ejecución (Importante para Waybar/Hyprland)
    make_scripts_executable()
    
    # 4. Refresco de cache de fuentes
    print("font-cache: Refrescando...")
    subprocess.run(["fc-cache", "-f"], stdout=subprocess.DEVNULL)
    
    print("\n✨ ¡Sistema listo y funcional!")

if __name__ == "__main__":
    main()
