#!/bin/zsh

if [[ $ZSH_EVAL_CONTEXT != 'toplevel:file' ]]; then
    echo "Error: Source the script instead of executing it:"
    echo
    echo "source $0"
    return 1 2>/dev/null || exit 1
fi

clai_TMP_FILE=$(mktemp)

if clai "$@" > $clai_TMP_FILE; then
  print -rz "$(< "$clai_TMP_FILE")"
else
  echo "$(< "$clai_TMP_FILE")"
fi
rm "$clai_TMP_FILE"

