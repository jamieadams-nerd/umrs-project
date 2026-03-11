
if [ -n "$BASH_VERSION" ]; then
    SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
elif [ -n "$ZSH_VERSION" ]; then
    SCRIPT_DIR="${0:A:h}"
fi
REL_PATH_BACK=$(realpath --relative-to="$PWD" "$SCRIPT_DIR")
echo $SCRIPT_DIR
echo $REL_PATH_BACK

##############################################################################


if [ ! -d src/ ]; then
    echo "Only run this at the top of the crate."
    exit 1
fi

CRATE_NAME=$(/usr/bin/basename ${PWD})
echo "Crate '${CRATE_NAME}': setting up..."

##############################################################################
# Functions to create required stuff.
##############################################################################

setup_selinux() {
    local CRATE

    # Strip off the umrs- prefix for selinux files. It already uses it.
    CRATE="${CRATE_NAME#umrs-}"

    if [ -d selinux/ ]; then 
        echo "[ Found ] selinux/"
    else
        echo "[Created] selinux/"
        mkdir selinux
    fi
    
    if [ ! -f selinux/${CRATE_NAME}.te ]; then
       cat ${REL_PATH_BACK}/selinux_te.template > selinux/${CRATE_NAME}.te
       /usr/bin/sed -i "s|CRATE|$CRATE|g" selinux/${CRATE_NAME}.te
       echo "    - Created selinux/${CRATE_NAME}.te for type-enforcements."
    fi

    if [ ! -f selinux/${CRATE_NAME}.fc ]; then
       cat ${REL_PATH_BACK}/selinux_fc.template > selinux/${CRATE_NAME}.fc
       /usr/bin/sed -i "s|CRATE|$CRATE|g" selinux/${CRATE_NAME}.fc
       echo "    - Created selinux/${CRATE_NAME}.fc for file contexts"
    fi
}


# Top level files first.

if [ ! -f README.md ]; then
    echo "# ${CRATE_NAME}" > README.md
    echo "[Created] README.md"
else
    echo "[ Found ] README.md"
fi

if [ ! -f .gitignore ]; then
    echo "target/" > .gitignore
    echo "[Cpdated] .gitignore"
else
    echo "[ Found ] .gitignore"
fi

# Functions to create stuff..
setup_selinux



# LICENSE
# CONTRIBUTING.md
# CODE_OF_CONDUCT.md
# SECURITY.md
# CHANGELOG.md
# .editorconfig
# rustfmt.toml
# clippy.toml
# deny.toml

#mkdir tests benches examples resources schemas selinux 

# Alternatively, for larger ones:
#mkdir systemd icons packaging deb
