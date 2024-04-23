#!/bin/zsh

if [[ $ZSH_EVAL_CONTEXT != 'toplevel:file' ]]; then
    echo "Error: Source the script instead of executing it:"
    echo
    echo "source $0"
    return 1 2>/dev/null || exit 1
fi

hilfe_TMP_FILE=$(mktemp)

if hilfe "$@" > $hilfe_TMP_FILE; then
  print -rz "$(< "$hilfe_TMP_FILE")"
else
  echo "$(< "$hilfe_TMP_FILE")"
fi
rm "$hilfe_TMP_FILE"

